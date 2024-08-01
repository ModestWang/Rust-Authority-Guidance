/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 16:04:59
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 16:18:59
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 高级 Trait
 */

// $$ 泛型 与 关联类型 的区别

// $$ 泛型
// -- 每次实现 Trait 时，都需要指定具体的类型
// -- 可以为一个类型实现 Trait 多次(不同的泛型参数)

// $$ 关联类型
// -- 在 Trait 中指定一个关联类型，实现 Trait 时必须指定具体的类型
// -- 只能为一个类型实现 Trait 一次

use core::fmt;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// $$ 默认泛型参数
// -- 可以为泛型参数指定默认类型
// -- 当调用时不指定具体类型时，会使用默认类型
pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    fn add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

// $$ 运算符重载
// -- 通过实现 Add Trait 来重载 + 运算符
// -- 通过实现 AddAssign Trait 来重载 += 运算符
// -- 通过实现 Mul Trait 来重载 * 运算符
// -- 通过实现 Sub Trait 来重载 - 运算符
// -- 通过实现 Div Trait 来重载 / 运算符
// -- 通过实现 Rem Trait 来重载 % 运算符
// -- 通过实现 Neg Trait 来重载 - 运算符
// -- 通过实现 Not Trait 来重载 ! 运算符
// -- 通过实现 BitAnd Trait 来重载 & 运算符
// -- 通过实现 BitOr Trait 来重载 | 运算符
// -- 通过实现 BitXor Trait 来重载 ^ 运算符
// -- 通过实现 Shl Trait 来重载 << 运算符
// -- 通过实现 Shr Trait 来重载 >> 运算符

// $$ 完全限定语法
// -- <Type as Trait>::function
// -- 可以在任何调用函数或方法的地方使用
// -- 忽略从上下文推到的部分
// -- 通常用于解决模糊性问题(Rust 无法确定具体调用哪个 Trait 的方法)

// $$ supertrait
// -- Trait 可以继承其他 Trait，称为 supertrait
// -- 子 Trait 可以使用父 Trait 的方法
// -- 子 Trait 可以覆盖父 Trait 的方法

// $$ newtype 模式
// -- 通过定义一个结构体，包含一个元素，实现 Trait
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("{}", w);
}
