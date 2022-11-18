mod consts;

mod commands;
use commands::{
    init::initialize_package,
    add::add_dependency
};

use std::path::{PathBuf};
use clap::{App, Arg};

use std::fs::{self};

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
        _ => println!("Error: No subcommand was used"),
    }
}
