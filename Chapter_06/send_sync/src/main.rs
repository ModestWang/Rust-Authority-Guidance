/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 22:38:43
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 22:43:15
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Send 和 Sync trait
 */

// $$ Send:允许线程间转移所有权
// 实现 Send trait 的类型可在线程间转移所有权
// Rust 中几乎所有的类型都实现了Send
// 但 RC<T>没有实现 Send，它只用于单线程情景
// 任何完全由 Send 类型组成的类型也被标记为Send

// $$ Sync:允许多线程访问
// 实现 Sync trait 的类型可安全的在多线程中拥有其引用
// Rust 中几乎所有的类型都实现了Sync
// 任何完全由 Sync 类型组成的类型也被标记为Sync

// $$ 手动实现 Send 和 Sync 是不安全的
// Send 和 Sync 是 Rust 中的隐式 trait
// 无需手动实现，编译器会自动为满足条件的类型实现这两个 trait

fn main() {
    println!("Hello, world!");
}
