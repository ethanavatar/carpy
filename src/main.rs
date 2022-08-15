use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use clap::{App, Arg, SubCommand};
use toml;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

const MAIN_CONTENTS: &[u8] =
b"def main():
    print(\"Hello, world!\")

if __name__ == \"__main__\":
    main()
";

const TEST_SAMPLE_CONTENTS: &[u8] =
b"import pytest

def inc(x):
    return x + 1

@pytest.mark.xfail(reason = \"Bug with arithmetic\")
def test_answer():
    assert inc(3) == 5
";

const SETUP_PY_CONTENTS: &[u8] =
b"from setuptools import setup

if __name__ == \"__main__\":
    setup()
";

#[derive(Serialize, Deserialize)]
struct PyProject {
    #[serde(rename = "build-system")]
    build_system: BuildSystem,

    project: Project,
}

#[derive(Serialize, Deserialize)]
struct BuildSystem {
    requires: Vec<String>,

    #[serde(rename = "build-backend")]
    build_backend: String,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    version: String,
    authors: Option<Vec<String>>,
    description: String,
    readme: String,
    license: Option<HashMap<String, String>>,
    #[serde(rename = "requires-python")]
    requires_python: String,
    classifiers: Vec<String>,
}

fn initialize_package(path: PathBuf) -> Result<(), io::Error> {

    let mut contents = Vec::new();
    for entry in fs::read_dir(path.clone())? {
        contents.push(entry?.path());
    }
    if contents.len() > 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "Directory is not empty"));
    }

    let name = path.file_name().unwrap().to_str().unwrap();
    println!("Initializing package: {}", name);

    let source_dir = path.join("src");
    let module_dir = source_dir.join(name);

    fs::create_dir_all(module_dir.clone())?;
    File::create(module_dir.clone().join("__init__.py"))?;
    let mut main = File::create(module_dir.clone().join("main.py"))?;
    main.write(MAIN_CONTENTS)?;

    let tests_dir = path.join("tests");
    fs::create_dir(tests_dir.clone())?;
    File::create(tests_dir.clone().join("__init__.py"))?;
    let mut test_sample = File::create(tests_dir.clone().join("test_sample.py"))?;
    test_sample.write(TEST_SAMPLE_CONTENTS)?;

    let mut pyproject_toml = File::create(path.join("pyproject.toml"))?;
    let project = PyProject {
        build_system: BuildSystem {
            requires: vec!["setuptools>=61.0".to_string(), "wheel".to_string()],
            build_backend: "setuptools.build_meta".to_string(),
        },
        project: Project {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            authors: None,
            description: "".to_string(),
            readme: "".to_string(),
            license: None,
            requires_python: ">=3.7".to_string(),
            classifiers: vec!["Programming Language :: Python :: 3".to_string()],
        },
    };
    let project_toml = toml::to_string(&project).unwrap();
    pyproject_toml.write(project_toml.as_bytes())?;

    let mut setup_py = File::create(path.join("setup.py"))?;
    setup_py.write(SETUP_PY_CONTENTS)?;

    Ok(())
}

fn main() {
    let matches = App::new("carpy")
        .version("1.0")
        .author("Ethan Evans <ethanalexevans@gmail.com>")
        .about("A package creation tool for Python inspired by Cargo")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes a new package")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            let name_arg = matches.subcommand_matches("init").unwrap().value_of("name").unwrap();
            let path_raw = PathBuf::from(name_arg);

            if !path_raw.exists() {
                fs::create_dir(&path_raw).unwrap();
            }
            
            let path_string = path_raw.canonicalize().unwrap()
                .to_str().unwrap().to_string()
                .replace("\\\\?\\", "");

            let path = PathBuf::from(path_string);
            match initialize_package(path) {
                Ok(_) => println!("Done"),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("No subcommand was used"),
    }
}
