use std::{fs, print};
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    fn new(query: &str, file_path: &str) -> Self {
        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }
}

pub fn config_args(args: &Vec<String>) -> Result<Config, &'static str> {
    // we can use an Option<T> enum in order to check the length of the args vector
    // Option enum might be useful but as we are dealing with the lenght of the args vector and need to check if the length is is smaller than 3 then we need to panic !
    // So option<T> might be an overkill  here

    //Result<T,E> {
    // Ok(T) => perform operation or anything on T ,
    // Err(E) =>
    // }

    if args.len() < 3 {
        return Err("Missing arguments ");
    }

    let text = args.get(1).expect("Text is not present ");
    let file_path = args.get(2).expect("File path or file is not present ");

    // text will store the string literal whichn means
    // it is not the owener of the the string : "apex"
    // query    return Config {
    // Option<T> {
    // Some(T) => Some,
    // None => None,
    // }
    return Ok(Config::new(text, file_path));
}

// for the time being
// we are just printing the contents of the file
pub fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Failed to read file  / FIle is not present at root level ");

    // need to handle much more complex cases :
    // file is present but no permission to read the content
    // file is not present then we need to create the file !

    let result = search_case_insensitive(&config.query, &contents);
    println!("{:#?}", result);
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // need to implement the search logic
    // in order to find the line that has the query string

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
        continue;
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // convert the quyery into smaller case
    // and content into smaller case

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
        continue;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn one_result() {
        let query = "vishal";
        let contents = "\
                Rust:
                safe, fast, productive.
                Pick three.";

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
