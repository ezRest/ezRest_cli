extern crate clap;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{File, create_dir_all};
use std::process::Command;
use std::io::{self, Write};

fn main() {
    let matches = App::new("ezRest")
        .version("b-0.0.1")
        .author("Simon <simonpeters05@gmail.com>")
        .author("Gus <je mail>")
        .about("Cli executable for setting up an ezRest project.")
        .subcommand(SubCommand::with_name("make:route")
                    .about("Makes new route file by give name")
                    .arg(Arg::with_name("model")
                         .help("Makes a model file")
                         .takes_value(false)
                         .short("m"))
                    .arg(Arg::with_name("route")
                         .help("Makes a route file")
                         .takes_value(false)
                         .short("r"))
                    .arg(Arg::with_name("INPUT")
                         .help("Generates route and model file")
                         .index(1)))

        .arg(Arg::with_name("clone")
            .help("Clones the ezRest repo from github")
            .takes_value(false)
            .short("c")
            .long("clone"))
        .get_matches();

    if matches.is_present("clone") {
        let command_result = Command::new("git")
            .args(&["clone", "https://github.com/ezRest/ezRest", "."])
            .output()
            .expect("Something went wrong while cloning");

        io::stdout().write_all(&command_result.stdout).unwrap();
        io::stderr().write_all(&command_result.stderr).unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("make:route") {
        let model = matches.is_present("model");
        let route = matches.is_present("route");

        println!("Model value: {}", model);

        match matches.value_of("INPUT") {
            Some(inp) => make_files(inp, model, route),
            _ => println!("Invalid input value for make:route.")
        }
    }
}

fn make_files(route_name: &str, model_present: bool, route_present: bool) {
    let path = env::var("PWD").unwrap();
    if (!model_present && !route_present) || (model_present && route_present) {
        create_dir_all(format!("{}/{}", &path, "src/models")).unwrap();
        create_dir_all(format!("{}/{}", &path, "src/routes")).unwrap();
        File::create(format!("{}/{}/{}", &path, "src/routes", [route_name, "Routes.rs"].concat())).unwrap();
        File::create(format!("{}/{}/{}", &path, "src/models", [route_name, ".rs"].concat())).unwrap();
    } else if !model_present && route_present {
        create_dir_all(format!("{}/{}", &path, "src/routes")).unwrap();
        File::create(format!("{}/{}/{}", &path, "src/routes", [route_name, "Routes.rs"].concat())).unwrap();
    } else if model_present && !route_present {
        create_dir_all(format!("{}/{}", &path, "src/models")).unwrap();
        File::create(format!("{}/{}/{}", &path, "src/models", [route_name, ".rs"].concat())).unwrap();
    } else {
        panic!("Error while creating files");
    }
}
