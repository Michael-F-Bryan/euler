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
extern crate rayon;

const PACKAGE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

use std::fs;
use std::path::Path;
use std::io::Read;
use std::process::Command;
use std::sync::mpsc::channel;
use std::fmt::Write;

use ansi_term::Colour::*;
use clap::{Arg, App};
use rayon::prelude::*;

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

    let (tx, rx) = channel::<Output>();

    for i in (0..binaries.len()).into_par_iter() {
        let ref challenge = binaries[i];
        let out = challenge.execute(&opts);
        tx.send(out).unwrap();
    }

    let mut results: Vec<Output> = rx.iter().take(binaries.len()).collect();
    results.sort_by(|left, right| left.number.cmp(&right.number));
    let errors: Vec<&Output> = results.iter().filter(|e: &&Output| e.return_code != 0).collect();

    if !opts.quiet {
        for result in results.iter() {
            print!("{}", result.stdout);
        }
    }

    println!("--------");
    println!("{}", Green.bold().paint("Challenge summary"));

    let ms = results.iter()
        .filter(|e: &&Output| e.return_code == 0)
        .map(|e| e.running_time)
        .fold(time::Duration::seconds(0), |acc, e| acc + e)
        .num_milliseconds() as f64;
    println!("Total running time: {}ms", ms);
    println!("Number of challenges: {}", binaries.len());
    println!("Average time: {:.2}ms", ms / binaries.len() as f64);

    if !errors.is_empty() {
        let error_challenges: String = errors.iter()
            .map(|e: &&Output| e.name.clone())
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

    fn execute(&self, opts: &Options) -> Output {
        let mut stdout = String::new();

        // First print the challenge's name
        writeln!(stdout,
                 "{} {}",
                 Green.bold().paint("Running challenge:"),
                 self.name)
            .unwrap();

        // Then print a bit of description about the challenge
        if !opts.quiet {
            writeln!(stdout, "{}", self.read_docstring()).unwrap();
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
            writeln!(stdout, "{}", Blue.bold().paint("Solution:")).unwrap();
            let command_stdout = String::from_utf8(output.stdout).unwrap();
            writeln!(stdout, "{}", command_stdout).unwrap();
        } else {
            writeln!(stdout,
                     "{}",
                     Red.bold().paint("An error occurred during execution"))
                .unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            writeln!(stdout, "{}", Red.paint(stderr)).unwrap();
        }

        let duration = end - start;
        writeln!(stdout,
                 "{} {}ms",
                 Green.paint("Running time:"),
                 duration.num_milliseconds())
            .unwrap();

        writeln!(stdout, "").unwrap();
        Output::new(&self, stdout, duration, output.status.code().unwrap_or(0))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct Options {
    pub as_release: bool,
    pub quiet: bool,
}


/// A container for a single binary's results
struct Output {
    name: String,
    number: usize,
    stdout: String,
    running_time: time::Duration,
    return_code: i32,
}

impl Output {
    pub fn new(challenge: &Challenge,
               stdout: String,
               running_time: time::Duration,
               return_code: i32)
               -> Output {
        Output {
            name: challenge.name.clone(),
            number: challenge.number,
            stdout: stdout,
            running_time: running_time,
            return_code: return_code,
        }
    }
}
