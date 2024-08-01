/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 16:28:38
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 16:41:16
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 高级 类型
 */

// $$ 类型别名(类似于 C++ 的 typedef)
// -- 使用 type 关键字为类型创建别名
type Kilometers = i32;

// $$ Never 类型
// -- Rust 有一个叫做 ! 的特殊类型，它被称为 “never” 类型。这个类型用于表明函数从不返回
// -- 例如，panic! 宏会导致程序立即退出，所以 panic! 的返回值可以被标注为 never
// -- Never 类型可以被转换为任何其他类型(类似于 C++ 的 void*)

// $$ 动态大小类型
// -- Rust 有一个叫做 Sized 的特性，它用于指定类型的大小是否在编译时已知
// -- 例如，str 是一个动态大小类型，因为我们不知道字符串的长度，直到运行时
// -- 为了在编译时处理动态大小类型，Rust 使用了一个叫做 trait object 的特性
// -- 为了创建一个 trait object，我们可以使用 &dyn Trait 或 Box<dyn Trait> 类型
// $$ Sized Trait
// -- Sized:    Rust 默认情况下，泛型参数是 Sized 的，这意味着它们的大小在编译时是已知的
// -- ?Sized:   为了支持动态大小类型，我们可以使用 ?Sized 来标注泛型参数
fn generic<T>(_t: &T) {}
fn generic<T: Sized>(_t: T) {}
fn generic<T: ?Sized>(_t: &T) {}

fn main() {
    println!("Hello, world!");
}
