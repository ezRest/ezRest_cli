extern crate clap;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::File;

fn main() {
    let matches = App::new("ezRest")
        .version("b-0.0.1")
        .author("Simon <simonpeters05@gmail.com>")
        .author("Gus <je mail>")
        .about("Cli executable for setting up an ezRest project.")
        .subcommand(SubCommand::with_name("make:route")
                    .about("Makes new route file by give name")
                    .author("Simon <simonpeters05@gmail.com>")
                    .arg(Arg::with_name("INPUT")
                         .help("Generates route and model file")
                         .index(1)))
                    .arg(Arg::with_name("model")
                         .help("Makes a model file")
                         .short("m"))
                    .arg(Arg::with_name("route")
                         .help("Makes a route file")
                         .short("r"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("make:route") {
        let model = matches.is_present("model");
        let route = matches.is_present("route");

        match matches.value_of("INPUT") {
            Some(inp) => make_files(inp, model, route),
            _ => println!("Invalid input value for make:route.")
        }
    }
}

fn make_files(route_name: &str, model_present: bool, route_present: bool) {
    let path = env::var("PWD").unwrap();
    if !model_present && !route_present {
        let filename: &str = format!("{}/{}", path, "models/test.rs");
        File::create(filename);
    }
}
