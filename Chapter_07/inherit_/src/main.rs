/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 01:03:30
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 01:08:07
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 通过 Trait 实现继承
 */

pub trait Animal {
    fn eat(&self);
    fn run(&self);
}

pub struct Dog;

impl Animal for Dog {
    fn eat(&self) {
        println!("Dog is eating");
    }

    fn run(&self) {
        println!("Dog is running");
    }
}

pub struct Cat;

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat is eating");
    }

    fn run(&self) {
        println!("Cat is running");
    }
}

fn main() {
    let dog = Dog;
    dog.eat();
    dog.run();

    let cat = Cat;
    cat.eat();
    cat.run();
}
