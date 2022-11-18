// Some programs allow arguments and environment variables for the same configuration. In those cases, the programs decide that one or the other takes precedence. For another exercise on your own, try controlling case sensitivity through either a command line argument or an environment variable. Decide whether the command line argument or the environment variable should take precedence if the program is run with one set to case sensitive and one set to ignore case.

use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
};

pub struct Args {
    ignore_case: bool,
    save_output: bool,
}

impl Args {
    fn new() -> Args {
        Args {
            ignore_case: env::var("IGNORE_CASE").is_ok(),
            save_output: false,
        }
    }

    fn config_args(&mut self, args: &[String]) {
        for arg in args {
            if arg == "--ignore-case" {
                self.ignore_case = true;
                continue;
            }

            if arg == "--save-output" {
                self.save_output = true;
                continue;
            }
        }
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub args: Args,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let mut cli_args = Args::new();
        cli_args.config_args(args);

        Ok(Config {
            query,
            file_path,
            args: cli_args,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.args.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if config.args.save_output {
        fs::remove_file("output.txt")?;
    }

    for line in results {
        println!("{line}");

        if config.args.save_output {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("output.txt")?;

            file.write_fmt(format_args!("{line}\n"))?;
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
