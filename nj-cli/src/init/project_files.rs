use std::path::PathBuf;
use std::fs::{File, remove_file, read_to_string};
use std::io::Write;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    package: ConfigPackage,
    lib: Option<ConfigLib>,
    dependencies: Option<Dependencies>,
    build_dependencies: Option<Dependencies>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigPackage {
    name: String,
    version: String,
    authors: Vec<String>,
    edition: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigLib {
    #[serde(rename = "crate-type")]
    crate_type: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependencies {
    #[serde(rename = "node-bindgen")]
    node_bindgen: Option<Dependency>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependency {
    version: String,
    features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectFiles {
    dir: PathBuf,
}

impl ProjectFiles {
    pub fn new(dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let project_files = Self { dir }
            .add_build_rs()?
            .add_lib_rs()?
            .add_cargo_toml()?;

        Ok(project_files)
    }

    fn add_build_rs(self) -> Result<Self, Box<dyn std::error::Error>> {
        let mut build_rs = self.dir.clone();
        build_rs.push("build.rs");

        let mut file = File::create(&build_rs)?;
        file.write_all(
            b"fn main() { 
            node_bindgen::build::configure(); 
        }",
        )?;

        Ok(self)
    }

    fn add_lib_rs(self) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lib_rs = self.dir.clone();
        lib_rs.push("src/lib.rs");

        // remove existing file, if exists;
        if lib_rs.exists() {
            remove_file(&lib_rs)?;
        }

        const LIB_RS: &str = r##"
            use node_bindgen::derive::node_bindgen;
            
            struct MyObject {}

            #[node_bindgen]
            impl MyObject {
            
                #[node_bindgen(constructor)]
                fn new() -> Self {
                    Self {}
                }

                #[node_bindgen(name = "hello")]
                fn hello(&self) -> String {
                    "world".to_string()
                }
            }
        "##;

        let mut file = File::create(&lib_rs)?;
        file.write_all(LIB_RS.as_bytes())?;

        Ok(self)
    }

    fn add_cargo_toml(self) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cargo_toml = self.dir.clone();
        cargo_toml.push("Cargo.toml");

        let mut config: Config = toml::from_str(&read_to_string(&cargo_toml)?)?;

        // NOTE: Attempt to get the workspace version instead of this hard coded value;
        let version = "2.1.1".to_string();

        config.lib = Some(ConfigLib {
            crate_type: vec!["cdylib".to_string()],
        });

        config.dependencies = Some(Dependencies {
            node_bindgen: Some(Dependency {
                version: version.clone(),
                features: vec![],
            }),
        });

        config.build_dependencies = Some(Dependencies {
            node_bindgen: Some(Dependency {
                version,
                features: vec!["build".to_string()],
            }),
        });

        // Remove the old cargo toml file;
        remove_file(&cargo_toml)?;

        let mut file = File::create(cargo_toml)?;
        file.write_all(toml::to_string(&config)?.as_bytes())?;

        Ok(self)
    }
}
