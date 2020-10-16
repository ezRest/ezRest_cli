use clap::{App};

fn main() {
    let matches = App::new("ezRest").get_matches();

    match matches.subcommand() {
        ("make", Some(_)) => println!("LOLLER"),
        _ => println!("yeet")
    }
}
