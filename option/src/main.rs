/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-15 21:07:56
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:18:49
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: option
****************************************************************************************
*/

// Rust 没有Null，但是有Option
// Option 是一个枚举类型，它有两个值：Some 和 None
// Some 包含一个值，None 不包含值
// Option<T> 是一个泛型类型，T 是任意类型
// Option<T> 有一些方法可以使用，比如 unwrap()，它返回 Some(T) 中的值，如果是 None，则会 panic

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(i) => println!("x: {}", i),
        None => println!("x: None"),
    }

    match y {
        Some(i) => println!("y: {}", i),
        None => println!("y: None"),
    }

    // 注意：Option<T> 和 T（Some(T)）是不同的类型
    // let a: i8 = 5;
    // let b: Option<i8> = Some(5);
    // let sum = a + b; // error: mismatched types
}
