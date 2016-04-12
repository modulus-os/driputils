#![feature(rustc_private)]

use std::env;
use std::io::prelude::*;
use std::fs::File;

extern crate term;

mod help {
	use term;

    pub fn display_help() {
        let mut t = term::stdout().unwrap();

        t.fg(term::color::BRIGHT_CYAN).unwrap();
        println!("     ____       _       __
    / __ \\_____(_)___  / /
   / / / / ___/ / __ \
                  \\/ /
  / /_/ / /  / / /_/ / /___
 /_____/_/  /_/ .___/_____/
            /_/");
        t.fg(term::color::BRIGHT_WHITE).unwrap();
        println!("A tool for linking Drip modules\n");

        t.fg(term::color::BRIGHT_GREEN).unwrap();
        println!("Usage:");
        t.fg(term::color::BRIGHT_WHITE).unwrap();
        println!("	dripl <command> [file] [options]\n");

        t.fg(term::color::BRIGHT_GREEN).unwrap();
        println!("Options:");
        t.fg(term::color::BRIGHT_WHITE).unwrap();
        println!("	-o [file]	Specifies output file\n");

        t.fg(term::color::BRIGHT_GREEN).unwrap();
        println!("Commands:");
        t.fg(term::color::BRIGHT_WHITE).unwrap();
        println!("	link		Links a flat binary file as a Drip module\n");
    }

    pub fn error(diagnostic: String) {
		display_help();
        let mut t = term::stdout().unwrap();
        t.fg(term::color::BRIGHT_RED).unwrap();
        print!("Error: ");
        t.fg(term::color::BRIGHT_WHITE).unwrap();
        println!("{}", diagnostic);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        help::display_help();
        return;
    }

    match args[1].as_ref() {
        "link" => link(args.clone()),
        _ => {
            let mut diagnostic = String::from("Unknown command: ");
            diagnostic.push_str(args[1].as_ref());
            help::error(diagnostic);
        }
    }
}

fn link(args: Vec<String>) {
	let mut t = term::stdout().unwrap();

    if args.len() < 3 {
        help::error(String::from("File not specified"));
		return;
    }

    let f = File::open(args[2].clone());
	if f.is_err() {
		let mut diagnostic = String::from("File not found: ");
		diagnostic.push_str(args[2].as_ref());
		help::error(diagnostic);
		return;
	}

	let mut file = String::new();
	f.unwrap().read_to_string(&mut file);

	t.fg(term::color::BRIGHT_GREEN).unwrap();
	println!("Linking...");

    return;
}
