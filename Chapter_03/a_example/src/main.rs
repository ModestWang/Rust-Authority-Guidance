/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 21:16:47
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-30 12:33:53
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 一个简单的命令行程序(使用闭包和迭代器改进)
 */
use a_example::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = a_example::run(config) {
        eprintln!("Applicatiob error: {}", e);
        process::exit(1);
    }
}
