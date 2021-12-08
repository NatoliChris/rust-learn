use std::error::Error;
use std::fs;
use std::env;

// Configuration struct
pub struct Config {
    pub query: String,
    pub filename: String,
    pub sensitive: bool,
}

impl Config {
    pub fn parse(mut args: env::Args) -> Result<Config, &'static str> {
        // Remove the program name from arguments
        args.next();

        // Use the iterator to iterate through arguments.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a filename"),
        };

        let sensitive = env::var("TEST_SENSITIVE").is_err();

        Ok(Config { query, filename, sensitive})
    }
}

/// Run the main logic in the program
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {

    // Use the `?` to return an error.
    let contents = fs::read_to_string(cfg.filename)?;

    let res = if cfg.sensitive {
        search(&cfg.query, &contents)
    } else {
        search_insensitive(&cfg.query, &contents)
    };

    for l in res {
        println!("Match: {}", l);
    }

    Ok(())
}


// Search through the content for the query string
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.contains(query))
        .collect()
}

// Case insensitive search
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()[..]))
        .collect()
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "orld";
        let contents = "\
A whole
new world
Such magestic.";
        assert_eq!(vec!["new world"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Such";
        let contents = "\
A whole
new world
such glory
Such magestic.";
        assert_eq!(vec!["Such magestic."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "MaGeStIc";
        let contents = "\
A whole
new world
Such magestic.";
        assert_eq!(vec!["Such magestic."], search_insensitive(query, contents));

    }
}
