/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-28 19:59:08
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 17:29:30
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Result 用于处理错误，它是一个枚举，有两个变体：Ok 和 Err。
 */

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    //相当于：
    // let mut f = match File::open("hello.txt"){
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
    //相当于：
    // match to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

// 上面函数的简化版
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

fn main() {
    let result = read_username_from_file();
}
