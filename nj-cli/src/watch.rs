use std::process::Command;

pub fn check_cargo_watch() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("cargo").args(&["watch", "--help"]).output()?;

    if output.status.success() {
        println!("cargo watch is installed");
        Ok(())
    } else {
        println!("attempting to install cargo watch");
        // Cargo watch is not installed, attempt to install;
        Command::new("cargo").args(&["install", "cargo-watch"]).output()?;
        // Re-run check
        Ok(check_cargo_watch()?)
    }
}