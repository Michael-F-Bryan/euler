//! A binary that will find all completed challenges and run them.
//!
//! It works by first reading the /src/bin/ folder and finding any files
//! matching "challenge_*.rs". Then it will invoke `cargo run --bin {}`
//! for each challenge, while also printing out a bit of the challenge's
//! docstring for context.

extern crate regex;
extern crate ansi_term;
extern crate time;
extern crate clap;

const PACKAGE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

use std::fs;
use std::path::Path;
use std::io::Read;
use std::process::Command;

use ansi_term::Colour::*;
use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(Arg::with_name("release")
            .short("O")
            .long("release")
            .help("Compile in release mode (include all optimisations)"))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Supress unnecessary output"))
        .get_matches();

    let options = Options {
        as_release: matches.is_present("release"),
        quiet: matches.is_present("quiet"),
    };

    let binaries = get_binaries();
    execute_binaries(binaries, options);
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

    // Sort according to the challenge's number
    challenges.sort_by(|a, b| a.number.cmp(&b.number));
    challenges
}


fn execute_binaries(binaries: Vec<Challenge>, opts: Options) {
    // Make sure everything is compiled
    let mut base_command = Command::new("cargo");

    let cmd = if opts.as_release {
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
    let mut errors = vec![];

    for challenge in binaries.iter() {
        println!("--------");
        let (status, duration) = challenge.execute(&opts);
        if status == 0 {
            total_time_taken = total_time_taken + duration;
        } else {
            errors.push(challenge);
        }

    }

    println!("--------");
    println!("{}", Green.bold().paint("Challenge summary"));

    let ms = total_time_taken.num_milliseconds() as f64;
    println!("Total running time: {}ms", ms);
    println!("Number of challenges: {}", binaries.len());
    println!("Average time: {:.2}ms", ms / binaries.len() as f64);

    if !errors.is_empty() {
        let error_challenges: String = errors.iter()
        .map(|e| e.name.clone())
        .collect::<Vec<String>>()
        .join(" ");
        println!("{} {}", Red.bold().paint("Errors:"), error_challenges);
    }
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

    fn execute(&self, opts: &Options) -> (i32, time::Duration) {
        // First print the challenge's name
        println!("{} {}", Green.bold().paint("Running challenge:"), self.name);

        // Then print a bit of description about the challenge
        if !opts.quiet {
            println!("{}", self.read_docstring());
        }

        let path = format!("{}/target/{}/{}",
                           PACKAGE_ROOT,
                           if opts.as_release { "release" } else { "debug" },
                           self.name);

        // Then execute `cargo run --bin {}`
        let start = time::now();
        let output = Command::new(&path)
            .output()
            .expect(&format!("Failed to run {}", self.name));
        let end = time::now();

        if output.status.success() {
            println!("{}", Blue.bold().paint("Solution:"));
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}", stdout);
        } else {
            println!("{}", Red.bold().paint("An error occurred during execution"));
            let stderr = String::from_utf8(output.stderr).unwrap();
            println!("{}", Red.paint(stderr));
        }

        let duration = end - start;
        println!("{} {}ms",
                 Green.paint("Running time:"),
                 duration.num_milliseconds());

        println!("");
        (output.status.code().unwrap_or(0), duration)
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct Options {
    pub as_release: bool,
    pub quiet: bool,
}
