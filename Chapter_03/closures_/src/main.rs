/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 23:29:35
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-30 10:55:11
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 闭包
 */

// $$ 闭包：闭包是一个可以捕获其环境的匿名函数。可以通过使用 || 语法创建闭包，其语法和参数列表类似于函数。
// $$ 闭包的三种形式：
// -- FnOnce:  消费从周围作用域捕获的变量，闭包周围的作用域只能调用一次。
// -- FnMut:   获取可变的借用值，通过 &mut T 来获取 T 的所有权。
// -- Fn:      从周围作用域获取不可变的借用值，通过 &T 来获取 T 的所有权。
// 在指定 Fn trait bound 时，首先使用 Fn, 如果编译器报错，再改为 FnMut，再报错，再改为 FnOnce。

use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            calculation: caculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // $$ 闭包的类型推断
    // 闭包的定义最终只会为参数/返回值类型推断唯一具体的类型。
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); // 会报错，因为闭包的参数类型推断为String，而不是i32（可以看作一次性的模板函数）
}

// 定义一个普通函数
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);

    // 定义一个匿名函数
    let expensive_closure = |num| {
        // 也可以手动标注返回值类型：let expensive_closure = |num| -> u32 {}
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Toody do{} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Today run for {} minutes!", expensive_closure(intensity))
        }
    }
}
