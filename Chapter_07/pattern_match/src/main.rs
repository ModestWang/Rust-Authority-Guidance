/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 01:12:24
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 01:32:48
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 模式匹配
 */

// $$ 模式: 是 Rust 中的一种特殊语法，用于检查一个值是否符合某种形式，并在符合时解构该值。
// $$ 模式由以下元素(或组合)构成：
// 1. 字面量
// 2. 解构的数组、枚举、结构体或元组
// 3. 变量
// 4. 通配符
// 5. 占位符

// $$ _  模式:  通配符，用于匹配任何值，但不绑定到变量。
// $$ .. 模式:  占位符，用于匹配范围的值。
// $$ |  模式:  或模式，用于匹配多个模式中的一个。
// $$ @  模式:  绑定符，用于在匹配时创建一个变量并绑定到匹配的值。

// $$ if let 表达式:    是一种结合模式匹配和条件表达式的语法，用于简化 match 表达式。
// $$ while let 表达式: 是一种结合模式匹配和循环的语法，用于简化 loop 循环。

// $$ for 循环中的模式是紧跟在 for 关键字后的部分

// $$ let PATTERN = EXPRESSION

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // match 表达式
    let x = 10;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // while let 表达式
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    while let Some(x) = v.pop() {
        println!("{}", x);
    }

    // let PATTERN = EXPRESSION
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);

    // @ 模式
    let point = Point { x: 10, y: 20 };
    match point {
        Point {
            x: x @ 0..11,
            y: y @ 0..21,
        } => println!("x = {}, y = {}", x, y),
        _ => println!("other"),
    }
}
