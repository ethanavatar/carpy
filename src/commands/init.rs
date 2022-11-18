use crate::consts::{
    MAIN_CONTENTS,
    TEST_SAMPLE_CONTENTS,
    SETUP_PY_CONTENTS,
};

use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use toml;
use std::collections::HashMap;

use std::fs::{self, File};
use std::io::Write;

use std::process::Command;



#[derive(Serialize, Deserialize)]
struct BuildSystem {
    requires: Vec<String>,

    #[serde(rename = "build-backend")]
    build_backend: String,
}

#[derive(Serialize, Deserialize)]
struct PyProject {
    #[serde(rename = "build-system")]
    build_system: BuildSystem,

    project: Project,
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

pub fn initialize_package(path: PathBuf) -> Result<(), Error> {

    let mut contents = Vec::new();
    for entry in fs::read_dir(path.clone())? {
        contents.push(entry?.path());
    }
    if contents.len() > 0 {
        return Err(Error::new(ErrorKind::Other, "Directory is not empty"));
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

    let mut _output = Command::new("git")
        .args(&["init", "."])
        .current_dir(path.clone())
        .output()
        .expect("Failed to initialize git repository");

    let mut _requirements = File::create(path.join("requirements.txt"))?;

    Ok(())
}