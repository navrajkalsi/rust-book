#![allow(unused)]

use minigrep::search;

use std::{env, error::Error, fs, process};

// use -- to pass anything after it to the binary and not to cargo
fn main() {
    let args: env::Args = env::args();
    // since .collect() can be used to create any collection, therefore
    // the type needs to be annotated explicitly
    let args: Vec<String> = args.collect();
    // args is an iterator of all the command line args
    // all args MUST BE made up of valid Unicode characters, otherwise args() panics

    // let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("query: {}", config.query);
    // println!("file_path: {}", config.file_path);

    // here we do not need the Ok variant, therefore using if let makes more sense
    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    // better way to error handle, as new should never fail
    // on error the error variant contains a string literal, denoted by static lifetime
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// this function will return a type that implements the Error trait, dyn stands for dynamic
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? returns Err for the whole function, and passes the Ok variant to contents
    let contents = fs::read_to_string(&config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
