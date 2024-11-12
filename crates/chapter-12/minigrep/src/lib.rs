use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;
    println!("{contents}");
    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("you must passing query and file parameter");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
