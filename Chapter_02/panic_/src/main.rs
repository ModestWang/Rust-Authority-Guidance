/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-28 20:20:41
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-28 22:53:34
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */
// $$ 总体原则（建议）
// 在定义一个可能失败的函数时，优先考虑返回 Result
// 否则使用 panic! 宏

// use std::net::IpAddr;

// fn main() {
//     let home: IpAddr = "127.0.0.1".parse().unwrap();
//     println!("{:?}", home);
// }

// // 猜数字游戏
// fn main() {
//     loop {
//         // 读取用户输入
//         println!("Please input your guess.");
//         let mut input = String::new();
//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         // 将输入转换为数字,并判断是否在1-100之间
//         let guess: i32 = match input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please input a number.");
//                 continue;
//             }
//         };
//         if guess < 1 || guess > 100 {
//             println!("The secret number will be between 1 and 100.");
//             continue;
//         } else if guess == 42 {
//             println!("You guessed it!");
//             break;
//         } else {
//             println!("Try again.");
//         }
//     }
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    loop {
        // 读取用户输入
        println!("Please input your guess.");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 将输入转换为数字
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        // 判断是否猜对
        let guess = Guess::new(guess);

        if guess.value() == 42 {
            println!("You guessed it!");
            break;
        } else {
            println!("Try again.");
        }
    }
}
