/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 13:12:50
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 13:13:33
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Box<T>类型:用于在堆上分配内存
 */

// $$ Box<T> 类型是一个智能指针，它允许你将值放在堆上而不是栈上。
// $$ 通过在堆上存储数据，当你想要在编译时未知大小的数据类型时，就可以编写出递归类型，而这种类型在编译时需要知道其大小。

// $$ Box<T> 属于预导入(prelude)的标准库类型，不需要使用 use 来引入。

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
