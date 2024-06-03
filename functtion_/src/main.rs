/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-12 10:52:43
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:17:48
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: function
****************************************************************************************
*/

fn main() {
    let x = 5;
    let y = 6;
    another_function(x, y);
    println!("{}", five());
    println!("{}", six());
}

fn another_function(x: i32, y: i32) {
    println!("Another function."); // 函数可以定义在任意位置
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // 默认返回值为最后一个值
}
fn six() -> i32 {
    return 6; // 也可以用return提前返回
}
