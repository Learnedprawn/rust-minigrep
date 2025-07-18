use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get query argument"),
        };
        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Didn't get file path argument"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in result {
        println!("Matched: {line}");
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
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick any three
Ducts are for losers";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick any three
Trust me.
Ducts are for losers";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}
