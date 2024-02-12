/*
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-12 23:27:30
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2024-02-12 23:39:18
 * 2024 by Modest Wang, All Rights Reserved.
 * @Descripttion:
 */

fn main() {
    let number = 3;

    // 单条件
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // 多条件（按顺序判断）
    if number % 3 == 0 {
        println!("number was something other than zero");
    } else if number % 4 == 0 {
        println!("number is divisible by 3");
    } else if number % 5 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("condition was false");
    }

    // 使用 if 来赋值
    let condition = true;
    let number = if condition { 5 } else { 6 }; // number 的类型必须一致
    println!("The value of number is: {}", number);
}
