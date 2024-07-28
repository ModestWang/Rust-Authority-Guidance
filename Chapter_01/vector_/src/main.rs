/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-16 16:09:36
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:16:45
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: vector
****************************************************************************************
*/

// let v: Vec<i32> = Vec::new();
// let v = vec![1, 2, 3]; // 使用宏创建一个Vec

fn main() {
    // 添加元素
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("v: {:?}", v);

    // 读取元素
    // 使用索引读取元素(返回&T)
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 使用get方法读取元素(返回Option<&T>)
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 遍历元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
