/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 16:42:49
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 16:47:31
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 高级 函数 和 闭包
 */

// $$ 函数指针
// -- 函数指针是一个指向函数的指针，它可以作为参数传递给其他函数
// -- 函数指针的类型是 fn(参数类型) -> 返回类型
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// $$ 函数指针与闭包的区别
// -- 函数指针是一个指向函数的指针，它是一个具体的类型
// -- 函数指针是实现了 Fn、FnMut 或 FnOnce trait，因此总是可以把函数指针作为参数传递给接收闭包的函数

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
