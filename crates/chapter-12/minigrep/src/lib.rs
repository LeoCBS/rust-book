use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for r in result {
        println!("{r}");
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

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* old style
        let query = query.to_lowercase();
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
        results
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            Nome => return Err("Didn't get a query string parameter"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            Nome => return Err("Didn't get a file path parameter"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive() {
        let query = "dUct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_insensitive(query, contents)
        );
    }
}
