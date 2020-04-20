#[cfg(windows)]
extern crate reqwest;

/// Slightly modified from https://github.com/Brooooooklyn/napi-rs/blob/master/build/src/lib.rs
/// configure linker to generate node.js dynamic library
pub fn configure() {
    if cfg!(target_os = "windows") {  // On Windows, we need to download the dynamic library from the nodejs.org website first
        use std::env::var;
        use std::fs::File;
        use std::io::copy;
        use std::path::PathBuf;
        use std::process::Command;

        let node_full_version = String::from_utf8(Command::new("node").arg("-v").output().unwrap().stdout).unwrap();
            
        let temp_dir =   PathBuf::from("C:\\Windows\\Temp\\");
        let mut temp_lib =   temp_dir.clone();
        
        temp_lib.push(format!("node-{}.lib", node_full_version.trim_end()));

        if !temp_lib.exists() {
            let lib_file_download_url = format!(
                "https://nodejs.org/dist/{}/win-x64/node.lib",
                node_full_version
            );
            let mut resp =
                reqwest::blocking::get(&lib_file_download_url).expect("Download node.lib file failed");
            let mut node_lib_file = File::create(&temp_lib).unwrap();
            copy(&mut resp, &mut node_lib_file).expect("Save node.lib file failed");
        }


        println!("cargo:rustc-link-lib={}",&temp_lib.file_stem().unwrap().to_str().unwrap());
        println!("cargo:rustc-link-search={}", temp_dir.to_str().unwrap());
        // Link `win_delay_load_hook.obj` for windows electron
        let node_runtime_env = "npm_config_runtime";
        println!("cargo:rerun-if-env-changed={}", node_runtime_env);
        if var(node_runtime_env).map(|s| s == "electron") == Ok(true) {
            println!("cargo:rustc-cdylib-link-arg=win_delay_load_hook.obj");
            println!("cargo:rustc-cdylib-link-arg=delayimp.lib");
            println!("cargo:rustc-cdylib-link-arg=/DELAYLOAD:node.exe");
        }
    } else if cfg!(target_os = "macos") {
        // Set up the build environment by setting Cargo configuration variables.
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }

    // On Linux, no additional configuration is needed
}