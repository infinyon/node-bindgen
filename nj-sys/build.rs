use bindgen::builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = builder()
        .header("node.h")
        .generate()
        .expect("Failed to generate bindings");
    bindings
        .write_to_file("src/binding.rs")
        .expect("Failed to write bindings");
    Ok(())
}
