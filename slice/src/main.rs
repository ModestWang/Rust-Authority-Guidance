/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-13 20:47:12
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:19:43
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: slice
****************************************************************************************
*/

fn main() {
    // 字符串切片（String Slice）
    // 【字符串切片是对字符串的引用，它并不拥有所有权】
    // 【字符串切片的类型是 &str】
    let s = String::from("hello world");
    let hello = &s[0..5]; // 从 0 开始，到 5 之前
    let world = &s[6..11]; // 从 6 开始，到 11 之前
    println!("hello: {}, world: {}", hello, world);

    // 字符串切片的另一种写法
    let s = String::from("hello world");
    let hello = &s[..5]; // 从 0 开始，到 5 之前
    let world = &s[6..]; // 从 6 开始，到结尾
    println!("hello: {}, world: {}", hello, world);

    // String可以很容易转换为&str
    // 所以，函数参数为&str的时候，具有更大的灵活性
}
