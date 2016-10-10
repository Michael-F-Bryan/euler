//! A binary that will find all completed challenges and run them.
//!
//! It works by first reading the /src/bin/ folder and finding any files
//! matching "challenge_*.rs". Then it will invoke `cargo run --bin {}`
//! for each challenge, while also printing out a bit of the challenge's
//! docstring for context.

extern crate regex;
extern crate ansi_term;
extern crate time;

const PACKAGE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

use std::fs;
use std::path::Path;
use std::io::Read;
use std::process::Command;

use ansi_term::Colour::*;


fn main() {
    let binaries = get_binaries();
    let as_release = true;
    execute_binaries(binaries, as_release);
}


/// Read the /src/bin/ directory and grab any file matching "challenge_*.rs".
fn get_binaries() -> Vec<Challenge> {
    let pattern = regex::Regex::new(r"challenge_(\d+).rs$").unwrap();
    let challenge_directory = Path::new(PACKAGE_ROOT).join("src/bin");

    let mut challenges = vec![];
    for dir_entry in fs::read_dir(challenge_directory).unwrap() {
        let path = dir_entry.unwrap().path();
        let path: &str = path.to_str().unwrap();

        if let Some(caps) = pattern.captures(path) {
            let num = caps.at(1).unwrap();
            let name = format!("challenge_{}", num);
            let c = Challenge {
                path: path.to_string(),
                name: name,
                number: num.parse().unwrap(),
            };
            challenges.push(c);
        };
    }

    // Sort according to the challenge's name
    challenges.sort_by(|a, b| a.number.cmp(&b.number));
    challenges
}


fn execute_binaries(binaries: Vec<Challenge>, as_release: bool) {
    // Make sure everything is compiled
    let mut base_command = Command::new("cargo");

    let cmd = if as_release {
        base_command.arg("build").arg("--release")
    } else {
        base_command.arg("build")
    };

    print!("{}", Green.paint("Re-compiling... "));
    let start = time::now();
    cmd.output().expect("Compilation failed! :(");
    let duration = time::now() - start;
    println!("compilation took {}ms", duration.num_milliseconds());
    println!("");

    let mut total_time_taken = time::Duration::seconds(0);

    for challenge in binaries.iter() {
        println!("--------");
        total_time_taken = total_time_taken + challenge.execute(as_release);
    }

    println!("--------");
    println!("{}", Green.bold().paint("Challenge summary"));

    let ms = total_time_taken.num_milliseconds() as f64;
    println!("Total running time: {}ms", ms);
    println!("Number of challenges: {}", binaries.len());
    println!("Average time: {:.2}ms", ms / binaries.len() as f64);
}


#[derive(Debug)]
struct Challenge {
    path: String,
    name: String,
    number: usize,
}

impl Challenge {
    /// Open up the file and grab the first 12 lines starting with "//!".
    fn read_docstring(&self) -> String {
        let mut f = fs::File::open(&self.path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

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

    fn execute(&self, as_release: bool) -> time::Duration {
        // First print the challenge's name
        println!("{} {}", Green.bold().paint("Running challenge:"), self.name);

        // Then print a bit of description about the challenge
        println!("{}", self.read_docstring());

        let path = format!("{}/target/{}/{}",
                           PACKAGE_ROOT,
                           if as_release { "release" } else { "debug" },
                           self.name);

        // Then execute `cargo run --bin {}`
        let start = time::now();
        let output = Command::new(&path)
            .output()
            .expect(&format!("Failed to run {}", self.name));
        let end = time::now();

        println!("{}", Blue.bold().paint("Solution:"));

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout);

        let duration = end - start;
        println!("{} {}ms",
                 Green.paint("Running time:"),
                 duration.num_milliseconds());

        println!("");
        duration
    }
}
