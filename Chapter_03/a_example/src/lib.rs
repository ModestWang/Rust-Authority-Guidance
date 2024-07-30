/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 22:33:11
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-30 12:39:55
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 将 main.rs 中的代码逻辑提取到 lib.rs 中
 */

use std::env;
use std::error::Error;
use std::fs;

// dyn Trait is likely to produce smaller code than impl Trait / generic parameters
// as the method won't be duplicated for each concrete type.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Error: not enough arguments.");
        }
        args.next();
        let query = args.next().unwrap();
        let filename = args.next().unwrap();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query = query.to_lowercase();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

// $$ 测试驱动开发(TDD)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Ru";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content));
    }
}
