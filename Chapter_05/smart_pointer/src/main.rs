/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 11:01:31
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 13:09:25
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 智能指针
 */

// $$ 智能指针是实现了 Deref trait 和 Drop trait 的类型
// - Deref trait    允许智能指针结构体实例表现得像引用一样，定义了智能指针的解引用行为
// - Drop trait     允许我们自定义当智能指针离开作用域时执行的代码

// $$ Box<T>
// - 用于在堆上分配内存(指针存放在 Stack 上，指向的数据存放在 Heap 上)

// $$ Rc<T>
// - 引用计数智能指针(需要在 heap 上分配数据，这些数据被程序的多个部分读取(只读)，但在编译时无法确定哪个部分最后使用完这些数据)
// - Rc<T> 允许多个所有者共享数据，但不提供并发操作(即 Rc<T> 只能用于单线程)

// $$ Refcell<T>
// - 用于在运行时而不是在编译时执行借用规则的类型

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil, // Nil 是一个 List 类型的空值，表示没有下一个元素
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    }

    #[test]
    fn test_deference() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
