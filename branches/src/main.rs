/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-12 23:27:30
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:17:13
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: branches
****************************************************************************************
*/

fn main() {
    // $$条件分支
    let number = 3;
    // 单条件
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
    // 多条件（按顺序判断）
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("condition was false");
    }
    // 使用 if 来赋值
    let condition = true;
    let number = if condition { 5 } else { 6 }; // number 的类型必须一致
    println!("The value of number is: {}", number);

    // $$循环
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    // while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    // for
    // for循环通常更加安全和简洁
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // for range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
