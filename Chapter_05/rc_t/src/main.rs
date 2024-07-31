/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 13:12:09
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 13:18:59
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Rc<T>类型:引用计数智能指针
 */

// $$ Rc<T>类型允许一个值有多个所有者，只有当没有任何所有者时，才会丢弃值
// Rc::clone(&a)        只会增加引用计数，而不会深拷贝数据
// Rc::strong_count(&a) 返回 Rc<T> 的强引用计数
// Rc::weak_count(&a)   返回 Rc<T> 的弱引用计数

// $$ Rc<T> 不是与导入到预导入(prelude)的标准库类型，所以需要使用 use 来引入。
use std::rc::Rc;

// 例：两个 List 共享另一个 List 的所有权
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 使用 Box<T> 类型的 List
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); //! error: value used here after move

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c get out of scope = {}", Rc::strong_count(&a));
}

// $$ 关于强引用和弱引用
// 强引用：Rc<T> 类型的引用，会增加强引用计数
// 弱引用：Weak<T> 类型的引用，不会增加强引用计数
// Rc::downgrade(&a) ：返回一个 Weak<T> 类型的引用
// Rc::upgrade(&a)   ：返回一个 Option<Rc<T>> 类型的引用
// 当强引用计数为 0 时，Rc<T> 类型的值会被丢弃。
