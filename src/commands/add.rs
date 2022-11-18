use std::path::PathBuf;

use std::fs::{File};
use std::io::{self, Read, Write};

use std::process::Command;

pub fn add_dependency(name: &str, project_root: PathBuf, create: bool) -> io::Result<()> {  

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
        deps.append(&mut contents.lines().collect::<Vec<&str>>());

        // `requirements` is dropped here
    }

    println!("Installing package: {}", name);

    // No need to save the result of this command because it is just to make sure the package is installed
    // TODO: ability to change the python command
    Command::new("python3")
        .args(&["-m", "pip", "install", name])
        .current_dir(project_root.clone())
        .output()
        .expect("Failed to install dependency");

    println!("freezing");
    // Save the whole list of currently installed packages
    // TODO: ability to change the python command
    let freeze = Command::new("python3")
        .args(&["-m", "pip", "freeze"])
        .current_dir(project_root.clone())
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