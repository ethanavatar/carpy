mod consts;
use consts::{*};

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{PathBuf};
use clap::{App, Arg};
use toml;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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

fn add_dependency(name: &str, project_root: PathBuf, create: bool) -> io::Result<()> {  

    let requirements_path = project_root.join("requirements.txt");

    if create && !requirements_path.exists() {
        println!("Creating requirements.txt (--create)");

        // Temporary file object to clear its contents
        //     requirements_path cloned to avoid conflicts with the move that occurs when the file is opened for work
        let file = File::create(requirements_path.clone())?;
        file.set_len(0)?;

        // `file` is dropped here
    }

    let mut contents: String = String::new();
    let mut deps: Vec<&str> = vec![];

    // Open new scope to read the file contents
    {
        let mut requirements = File::options()
            .read(true)
            .write(true)

            // requirements_path moved here
            .open(requirements_path)?;

        // Read the contents of the file
        requirements.read_to_string(&mut contents)?;

        // appending to avoid a compiler warning about unused initializations
        deps.append(&mut contents.lines().collect());

        // `requirements` is dropped here
    }

    println!("Installing package: {}", name);

    // No need to save the result of this command because it is just to make sure the package is installed
    Command::new("python3")
        .args(&["-m", "pip", "install", name])
        .current_dir(path.clone())
        .output()
        .expect("Failed to install dependency");

    println!("freezing");
    // Save the whole list of currently installed packages
    // TODO: ability to change the python command
    let freeze = Command::new("python3")
        .args(&["-m", "pip", "freeze"])
        .current_dir(path.clone())
        .output()
        .expect("Failed to freeze dependencies")
        .stdout;

    // freeze moved here. Shadowed to an owned String
    let freeze = String::from_utf8(freeze).unwrap();
    let freeze: Vec<&str> = freeze.lines().collect();

    // Iterate over the list of installed packages
    freeze.into_iter()
        // Filter out the packages that are not the one we want to add
        .filter(|dep| dep.starts_with(name))

        // add the desired package to the list of dependencies
        .for_each(|dep| deps.push(dep));

    // Clean up duplicates an empty lines
    deps.dedup();
    deps.retain(|&x| x != "");
    println!("{:?}", deps);

    // Write the new list of dependencies to the file

    let write_out = deps.join("\n");
    
    let mut requirements = File::options()
        .write(true)

        // clear the file contents
        .truncate(true)
        .open("requirements.txt")?;

    requirements.write_all(write_out.as_bytes())?;

    Ok(())
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

    let mut _output = Command::new("git")
        .args(&["init", "."])
        .current_dir(path.clone())
        .output()
        .expect("Failed to initialize git repository");

    let mut _requirements = File::create(path.join("requirements.txt"))?;

    Ok(())
}

fn main() {
    let matches = App::new("carpy")
        .version("1.0")
        .author("Ethan Evans <ethanalexevans@gmail.com>")
        .about("A package creation tool for Python inspired by Cargo")
        .subcommand(
            clap::Command::new("init")
                .help("Initialize a new package")
                .arg(Arg::new("name").required(true).index(1)),
        )
        .subcommand(
            clap::Command::new("add")
                .help("Adds a dependency to the package")
                .arg(Arg::new("name").required(true).index(1))
                .arg(Arg::new("create")
                    .long("create")
                    .help("Creates the requirements.txt file if it doesn't exist")
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            let name_arg = matches.subcommand_matches("init").unwrap().value_of("name").unwrap();
            let path = PathBuf::from(name_arg);

            if !path.exists() {
                fs::create_dir(&path).unwrap();
            }

            match initialize_package(PathBuf::from(path)) {
                Ok(_) => println!("Done"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Some("add") => {
            let create_flag = matches.subcommand_matches("add").unwrap().is_present("create");
            let name_arg = matches.subcommand_matches("add").unwrap().value_of("name").unwrap();

            println!("Adding dependency: {}, create = {}", name_arg, create_flag);

            match add_dependency(name_arg, PathBuf::from("."), create_flag) {
                Ok(_) => println!("Done"),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("No subcommand was used"),
    }
}
