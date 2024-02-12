use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // println! 是一个宏，宏是一种在 Rust 中提供的一种在编译时生成代码的方式
    println!("猜数游戏！");

    // 生成一个1-100的随机数
    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("秘密数字是：{}", secret_number);

    // loop 关键字创建了一个无限循环
    loop {
        println!("猜一个数：");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行！");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break; // 退出循环
            }
        }
    }
}
