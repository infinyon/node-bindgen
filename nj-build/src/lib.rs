mod win_delay_load_hook;

/// Slightly modified from https://github.com/Brooooooklyn/napi-rs/blob/master/build/src/lib.rs
/// configure linker to generate node.js dynamic library
#[cfg(windows)]
pub fn configure() {
    // On Windows, we need to download the dynamic library from the nodejs.org website first
    use std::env::var;
    use std::fs::File;
    use std::io::copy;
    use std::process::Command;
    use std::env::temp_dir;

    let node_full_version = String::from_utf8(Command::new("node").arg("-v").output().unwrap().stdout).unwrap();
    
    let tmp_dir = temp_dir();
    let temp_lib = tmp_dir.clone().join(format!("node-{}.lib", node_full_version.trim_end()));

    if !temp_lib.exists() {

        let lib_file_download_url = format!(
            "https://nodejs.org/dist/{}/win-x64/node.lib",
            node_full_version
        );

        println!("downloading nodejs: {} to: {:#?}",lib_file_download_url,temp_lib);

        let mut resp =
            reqwest::blocking::get(&lib_file_download_url).expect("Download node.lib file failed");
        let mut node_lib_file = File::create(&temp_lib).unwrap();
        copy(&mut resp, &mut node_lib_file).expect("Save node.lib file failed");
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
        win_delay_load_hook::build(tmp_dir).expect("Failed to build win_delay_load_hook");

        println!("cargo:rustc-cdylib-link-arg=win_delay_load_hook.o");
        println!("cargo:rustc-cdylib-link-arg=delayimp.lib");
        println!("cargo:rustc-cdylib-link-arg=/DELAYLOAD:node.exe");
    }
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
