use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
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

    fn config_args(&mut self, args: impl Iterator<Item = String>) {
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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

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
        save_output(&results)?;
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(lowercase_query.as_str()))
        .collect()
}

fn save_output(results: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    fs::remove_file("output.txt").ok();

    for line in results {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("output.txt")?;

        file.write_fmt(format_args!("{line}\n"))?;
    }

    Ok(())
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

    #[test]
    fn create_file_and_save_output() {
        let results = vec![
            "Lorem ipsum dolor sit amet",
            "consectetur adipiscing elit",
            "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
            "sunt in culpa qui officia",
        ];

        let response = save_output(&results);

        match response {
            Ok(_) => (),
            Err(error) => panic!("Problem on save_output: {}", error),
        }
    }
}
