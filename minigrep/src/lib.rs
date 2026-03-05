use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
// fn judge_from_cmd(args: &[String]) -> bool{
fn is_not_sensitive_from_command_line(mut args: std::env::Args) -> bool{
    match args.next() {
        Some(s) => s.contains("CASE_INSENSITIVE"),
        None => false
    }
}
// fn judge(args: &[String]) -> bool{
fn is_not_sensitive_from_env(args: std::env::Args) -> bool{
    match is_not_sensitive_from_command_line(args) {
        true => true ,
        false => env::var("CASE_INSENSITIVE").is_err()
    }
}
impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static  str> {
        args.next();
        // let query = args[1].clone();
        // let filename = args[2].clone();
        let query = args.next().ok_or("not enough arguments")?;
        let filename = args.next().ok_or("not enough arguments")?;
        let case_sensitive = is_not_sensitive_from_env(args);
        // if casesensitive_on_cmd.contains("CASE_INSENSITIVE") { case_sensitive = false; }
        Ok(Config{
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents =
        fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("With text: {}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(| line | line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let case_insensitive = &query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(case_insensitive){results.push(line);}
    };
    results

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
        assert_eq!(vec!["safe, fast, productive."],search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],search_case_insensitive(query, contents))
    }

    #[test]
    fn test123(){
        // let args :Vec<String> = env::args().collect();
        // let query = &args[1];
        let query = String::from("CLION_VM_OPTIONS");
        let sn = env::var(&query).unwrap_or_else(|_| {String::from("NOT found.")});
        println!("sn num :: {}",sn);
    }

}
