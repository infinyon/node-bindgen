use std::process::Command;
use structopt::StructOpt;
use std::process::Stdio;

#[derive(Debug, StructOpt)]
pub struct WatchOpt {
    extras: Vec<String>,
}

pub fn check_cargo_watch() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("cargo").args(&["watch", "--help"]).output()?;

    if output.status.success() {
        println!("cargo watch is installed");
        Ok(())
    } else {
        println!("installing cargo watch... this might take a minute.");
        // Cargo watch is not installed, attempt to install;
        Command::new("cargo").args(&["install", "cargo-watch"]).output()?;
        // Re-run check
        println!("checking cargo watch installation...");
        Ok(check_cargo_watch()?)
    }
}

pub fn run(opt: WatchOpt) {
    if check_cargo_watch().is_ok() {
        // Use cargo watch to monintor files
        let mut args = vec!["watch".to_string()];

        // Pass in extra 
        args.extend(opt.extras);

        // Start watching files;
        let mut watch = Command::new("cargo")
            .args(&args)
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute command");
        
        // Wait on the child process;
        watch.wait()
            .expect("failed to wait on child");
    }
}