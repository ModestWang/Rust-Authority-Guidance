/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-27 16:07:25
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-27 20:16:40
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */
// $$HashMap 相当于 Python 中的字典(dict)，可以用来存储键值对
// HashMap 是标准库中的一部分，不在 prelude 中，所以需要手动引入
// HashMap 是同构的，即所有的键类型相同，所有的值类型相同
use std::collections::HashMap;

fn main() {
    // 创建一个空的 HashMap
    let mut scores = HashMap::new();

    // 插入键值对
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // 使用 collect 方法将两个 vector 合并成一个 HashMap
    // let teams: Vec<String> = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores: Vec<i32> = vec![10, 50];
    // let _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // get 方法获取给定键对应的值: 返回一个 Option<&V>
    match scores.get("blue") {
        Some(score) => println!("blue score: {}", score),
        None => println!("blue score not found"),
    }

    // insert 方法会覆盖之前的值
    scores.insert(String::from("blue"), 25);
    println!("{:?}", scores);

    // entry 方法检查给定的键是否有对应的值，如果没有则插入
    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("red")).or_insert(50);
    println!("{:?}", scores);

    // 例子：统计一个字符串中每个单词出现的次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert 方法返回这个键的值的一个可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
