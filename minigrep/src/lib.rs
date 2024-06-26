
use::std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {  // here () is the unit type, which is a type that has only one value, an empty tuple
  let contents = fs::read_to_string(config.file_path)?;
  // println!("With text:\n{contents}");

  let results = if config.ignore_case {
      search_case_insensitive(&config.query, &contents)
  } else {
      search(&config.query, &contents)
  };

  for line in results {
      println!("{line}");
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool
}

impl Config {
  // need to specify the lifetime of the string slice returned by env::Args
  // earlier we were able to use &str without specifying the lifetime because we had a reference to a string slice and the return values got their lifetime from the reference
  // here we are returning a string slice directly ie. owned, so we need to specify the lifetime
  // static lifetime is the lifetime of the entire program
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };

    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config {
        query,
        file_path,
        ignore_case,
    })
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // .lines() returns an iterator over the lines of the string
  // .filter() creates an iterator that only yields elements of the input iterator that satisfy a specified condition
  // .collect() consumes the iterator and collects the resulting values into a collection
  contents
  .lines()
  .filter(|line| line.contains(query))
  .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
      if line.to_lowercase().contains(&query) { // to_lowercase() returns a new String
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
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
        );
    }
}