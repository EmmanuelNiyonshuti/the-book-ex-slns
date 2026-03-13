use std::{env, fs, process};
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    */

    //now, rather than collecting iterator values into a vector
    //and passing a slice in Config::build we can
    // pass ownership of the iterator to Config::build directly and now it will take ownership
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case { 
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}


impl Config {
    /*
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }*/

    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // we need to ignore the first value, which is the name of the programme

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
    
}

