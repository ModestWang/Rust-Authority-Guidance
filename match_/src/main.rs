/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-15 21:38:19
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:18:30
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: match
****************************************************************************************
*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Statee quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("The value of the coin is: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);

    // 可以使用通配符 "_" 来匹配所有其他情况
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        _ => (),
    }

    // 拓展：if let
    // if let 可以用来简化match的一部分情况
    // 用于只关心一种匹配而忽略其他匹配情况
    let some_u8_value = Some(0u8);
    if let Some(1) = some_u8_value {
        println!("one");
    } else {
        println!("not three");
    }
    // 这段 if let 代码与上面的 match 代码等价
    // 可以将 if let 看作是 match 的一个特例/语法糖
}

// match匹配时，必须穷尽所有可能性，否则会报错
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
