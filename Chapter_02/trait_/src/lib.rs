/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 10:12:32
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 17:20:05
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Trait（特质特征）是一种定义共享行为的方法，类似于其他语言中的接口。
 */

use std::fmt::Debug;
use std::fmt::Display;

// $$ 定义 trait
pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    } // 默认实现
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub struct Wechat {
    pub username: String,
    pub content: String,
}

// $$ 实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Wechat {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// $$ trait 作为参数: 参数必须是实现了 trait 的类型（对参数进行了约束）
// impl Trait 语法可以用在返回 trait 的函数中，以返回不同类型的实例（简单情况）。
// Trait bound 语法可以用在函数签名中，以接受不同类型的实例（复杂情况）。

// impl Trait 语法
pub fn notify_1(item1: impl Summary, item2: impl Summary) {
    println!("Good news! {}", item1.summarize());
    println!("Good news! {}", item2.summarize());
}

// Trait bound 语法
pub fn notify_2<T: Summary>(item1: T, item2: T) {
    println!("Good news! {}", item1.summarize());
    println!("Good news! {}", item2.summarize());
}
// + 运算符允许我们指定多个 trait。
pub fn notify_3<T: Summary + Display, U: Clone + Debug>(t: T, u: T) -> String {
    format!("Good news! {}", t.summarize())
}

// where 从句: 使函数签名更易读
pub fn notify_4<T, U>(t: T, u: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Good news! {}", t.summarize())
}
