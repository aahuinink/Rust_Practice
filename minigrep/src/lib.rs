use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

pub struct Config 
{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() == 3 {
            let query = args[1].clone();
            let file_path = args[2].clone();
            let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

            return Ok(Config { query, file_path, ignore_case})
        }
        
        Err("Please provide a search term and a file path")
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}