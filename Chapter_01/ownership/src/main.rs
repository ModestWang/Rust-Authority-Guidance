/**
****************************************************************************************
 * @FilePath: main.rs
 * @Author: Modest Wang 1598593280@qq.com
 * @Date: 2024-02-13 20:46:50
 * @LastEditors:
 * @LastEditTime: 2024-02-16 16:19:10
 * @2024 by Modest Wang, All Rights Reserved.
 * @Descripttion: ownership
****************************************************************************************
*/

// Stack and Heap
// Stack: LIFO, fixed size, fast, data must have a known size at compile time
// Heap: dynamic size, slower, data can grow or shrink, data size need not be known at compile time

// 所有权规则
// 1. Rust 中的每一个值都有一个被称为其所有者的变量。
// 2. 值在任一时刻有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。

fn main() {
    let mut s = String::from("hello"); // s 是有效的直到作用域结束
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
                            // 字符串字面值是不可变的，在编译时就已知其内容
                            // 而 String 是可变的，需要在 Heap (堆)上分配内存来保存编译时未知的文本内容
    println!("{}", s); // 将打印 `hello, world!`

    // 变量和数据的交互方式：移动（move）
    // 移动（Move）：【当你把一个值赋给另一个变量时，原来的变量将不再拥有这个值】
    // 例如，let s2 = s1; 这行代码，s1 的值被移动到 s2 中，s1 将不再有效。
    let s1 = String::from("hello");
    let s2 = s1; // s1 的值被移动到 s2 中
                 // println!("{}", s1); // 这里会报错，因为 s1 的值已经被移动到 s2 中
    println!("{}", s2); // 这里会打印 `hello`

    // 变量和数据的交互方式：克隆（clone）
    // 克隆（Clone）：【如果你想要复制一个值而不是移动，你可以使用 .clone() 方法】
    // 例如，let s2 = s1.clone(); 这行代码，s1 的值被复制到 s2 中，s1 和 s2 都是有效的。
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆 s1 的数据到 s2
    println!("s1 = {}, s2 = {}", s1, s2); // 这里会打印 `s1 = hello, s2 = hello`

    // 变量和数据的交互方式：引用（reference）
    // 引用（Reference）：【如果你想要使用一个值但不取得其所有权，你可以使用引用】
    // 引用允许你使用值但不取得其所有权，这意味着值在你使用完之后不会被丢弃。
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递 s1 的引用
    println!("The length of '{}' is {}.", s1, len); // 这里会打印 `The length of 'hello' is 5`

    // 可变引用（mutable reference）
    let mut s = String::from("hello");
    change(&mut s); // 传递 s 的可变引用
    println!("{}", s); // 这里会打印 `hello world`

    // 悬垂引用（dangling reference）
    // let reference_to_nothing = dangle(); // 这里会报错，因为 dangle 返回的引用指向了一个局部变量

    // 引用的规则
    // 1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // 2. 引用必须总是有效的。
    // 3. 引用不会被 Rust 编译器隐式地添加。
}
// 当变量走出作用域时，Rust 会调用一个特殊的函数 drop 来释放内存

fn calculate_length(s: &String) -> usize {
    // s 是一个指向 String 的引用
    s.len()
} // 这里 s 离开了作用域，但因为它并不拥有引用值的所有权，所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}
