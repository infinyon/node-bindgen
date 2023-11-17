mod win_delay_load_hook;

#[cfg(windows)]
mod win {
    use super::win_delay_load_hook;
    use std::env::var;
    use std::fs::{File, remove_file};
    use http_req::request;
    use std::{
        process::Command,
        env::temp_dir,
        io,
        path::PathBuf,
        fs::{read_dir, create_dir, DirEntry},
        time::{SystemTime, UNIX_EPOCH},
    };

    macro_rules! cargo_warn {
        ($($tokens: tt)*) => {
            println!("cargo:warning={}", format!($($tokens)*))
        }
    }

    fn print_folder(path: &PathBuf) {
        match ls(path) {
            Ok(list) => {
                list.iter()
                    .for_each(|entry| cargo_warn!("{:?}", entry.path()));
            }
            Err(err) => cargo_warn!("Fail read {path:?}: {err}"),
        }
    }

    fn ls(path: &PathBuf) -> Result<Vec<DirEntry>, io::Error> {
        let mut list = vec![];
        for entry in read_dir(path)? {
            list.push(entry?);
        }
        Ok(list)
    }

    fn tmp_folder_name() -> String {
        format!(
            "node_bindgen_build_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos()
        )
    }

    fn find_file_ends_with(path: &PathBuf, filename: &str) -> Option<String> {
        let list = ls(path).expect("Get list of files from temp folder");
        list.iter()
            .find(|entry| {
                if let Ok(mt) = entry.metadata() {
                    if mt.is_file() {
                        return entry.file_name().to_string_lossy().ends_with(filename);
                    }
                }
                false
            })
            .map(|entry| entry.file_name().to_string_lossy().to_string())
    }

    pub fn configure() {
        // On Windows, we need to download the dynamic library from the nodejs.org website first
        let node_full_version =
            String::from_utf8(Command::new("node").arg("-v").output().unwrap().stdout)
                .unwrap()
                .trim_end()
                .to_string();

        let tmp_dir = temp_dir().join(tmp_folder_name());
        if !tmp_dir.exists() {
            create_dir(&tmp_dir).expect("Folder {tmp_dir:?} would be created");
        }
        let temp_lib = tmp_dir.join(format!("node-{}.lib", node_full_version));

        if !temp_lib.exists() {
            let lib_file_download_url = format!(
                "https://nodejs.org/dist/{}/win-x64/node.lib",
                node_full_version
            );

            cargo_warn!(
                "downloading nodejs: {} to: {:#?}",
                lib_file_download_url,
                temp_lib
            );

            let mut node_lib_file = File::create(&temp_lib).unwrap();
            if let Err(err) = request::get(&lib_file_download_url, &mut node_lib_file) {
                if temp_lib.exists() {
                    if let Err(err) = remove_file(&temp_lib) {
                        cargo_warn!("Fail to remove {:#?} due error: {}", temp_lib, err);
                    }
                }
                panic!("Download node.lib file failed with: {}", err);
            };
        }

        println!(
            "cargo:rustc-link-lib={}",
            &temp_lib.file_stem().unwrap().to_str().unwrap()
        );
        println!("cargo:rustc-link-search={}", tmp_dir.to_str().unwrap());

        // Link `win_delay_load_hook.obj` for windows electron
        let node_runtime_env = "npm_config_runtime";
        println!("cargo:rerun-if-env-changed={}", node_runtime_env);

        if var(node_runtime_env).map(|s| s == "electron") == Ok(true) {
            // Build win_delay_load_hook
            let mut filename = format!(
                "{}.o",
                win_delay_load_hook::build(tmp_dir.clone())
                    .expect("Failed to build win_delay_load_hook")
            );
            let full_filename = tmp_dir.join(&filename);
            // Checking for object file
            if !full_filename.exists() {
                cargo_warn!("File {full_filename:?} doesn't exist");
                // Drop into logs list of generated files
                print_folder(&tmp_dir);
                // It might be a target file is generated, but the name includes a prefix
                cargo_warn!("Looking for file {filename} with some prefix in {tmp_dir:?}");
                if let Some(prefix_filename) = find_file_ends_with(&tmp_dir, &filename) {
                    filename = prefix_filename;
                    cargo_warn!("Found object file {filename}");
                } else {
                    panic!("Fail to find any related object file");
                }
            }
            println!("cargo:rustc-cdylib-link-arg={filename}");
            println!("cargo:rustc-cdylib-link-arg=delayimp.lib");
            println!("cargo:rustc-cdylib-link-arg=/DELAYLOAD:node.exe");
            println!("cargo:rustc-cdylib-link-arg=/INCLUDE:__pfnDliNotifyHook2");
            println!("cargo:rustc-cdylib-link-arg=/FORCE:MULTIPLE");
        }
    }
}

/// Slightly modified from https://github.com/Brooooooklyn/napi-rs/blob/master/build/src/lib.rs
/// configure linker to generate node.js dynamic library
#[cfg(windows)]
pub fn configure() {
    win::configure();
}

#[cfg(unix)]
pub fn configure() {
    if cfg!(target_os = "macos") {
        // Set up the build environment by setting Cargo configuration variables.
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }

    // On Linux, no additional configuration is needed
}
