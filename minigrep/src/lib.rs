use std::error::Error;
use std::fs;
use std::env;

pub struct GrepObject
{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl GrepObject {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<GrepObject, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(GrepObject { query, file_path, ignore_case })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents: String = if self.ignore_case {
            fs::read_to_string(&self.file_path)?.to_lowercase()
        } else {
            fs::read_to_string(&self.file_path)?
        };

        let query = if self.ignore_case {
            &self.query.to_lowercase()
        } else {
            &self.query
        };

        let result = search(&query, &contents);

        for line in result.iter() {
            println!("{line}");
        }

        Ok(())
    }
}

    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }