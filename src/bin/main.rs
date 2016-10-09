//! A binary that will find all completed challenges and run them.
#![feature(box_syntax)]

extern crate regex;
extern crate ansi_term;

const PACKAGE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

use std::fs;
use std::path::Path;
use std::io::Read;
use std::process::{Command, Output};

use ansi_term::Style;
use ansi_term::Colour::*;


fn main() {
    let binaries = get_binaries();
    execute_binaries(binaries);
}


/// Read the /src/bin/ directory and grab any file matching "challenge_*.rs".
fn get_binaries() -> Vec<Challenge> {
    let pattern = regex::Regex::new(r"(challenge_\d+).rs$").unwrap();
    let challenge_directory = Path::new(PACKAGE_ROOT).join("src/bin");

    let mut challenges = vec![];
    for dir_entry in fs::read_dir(challenge_directory).unwrap() {
        let path = dir_entry.unwrap().path();
        let path: &str = path.to_str().unwrap();

        match pattern.captures(path) {
            Some(caps) => {
                let c = Challenge {
                    path: path.to_string(),
                    name: caps.at(1).unwrap().to_string(),
                };
                challenges.push(c);
            }

            None => ()
        }
    }

    challenges
}


fn execute_binaries(binaries: Vec<Challenge>) {
    for challenge in binaries {
        challenge.execute();
    }
}


#[derive(Debug)]
struct Challenge {
    path: String,
    name: String,
}

impl Challenge {
    /// Open up the file and grab the first 12 lines starting with "//!".
    fn read_docstring(&self) -> String {
        let mut f = fs::File::open(&self.path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents);

        let mut docstring = contents.lines()
        .filter(|line| line.starts_with("//!"))
        .take(16)
        .fold(String::new(), |mut s, line| {
            s.push_str(line);
            s.push('\n');
            s
        });

        docstring = docstring.replace("//! ", "");
        docstring.replace("//!", "")

    }

    fn execute(&self) {
        // First print the challenge's name
        println!("--------");
        println!("{} {}", Green.bold().paint("Running challenge:"), self.name);

        // Then print a bit of description about the challenge
        println!("{}", self.read_docstring());

        // Then execute `cargo run --bin {}`
        let output = Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg(&self.name)
                .output()
                .expect(&format!("Failed to run {}", self.name));

        println!("{}", Style::new().bold().paint("Solution:"));

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout);
    }
}
