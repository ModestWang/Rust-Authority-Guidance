/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 11:24:15
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 11:51:58
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// $$ Deref trait
use std::ops::Deref; // 导入 Deref trait

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// $$ Drop trait
struct CustomSmartPoint {
    data: String,
}

// 实现 Drop trait
// 将在离开作用域时 执行 Drop trait 的 drop 方法
impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoint with data `{}`!", self.data);
    }
}

// $$ 注: Rust 不允许我们显式调用 Drop trait 的 drop 方法，因为这可能会导致双重释放内存
// $$ 如果想要提前释放内存，可以使用 std::mem::drop 函数

// $$ 测试代码
#[cfg(test)]
mod deref_tests {
    use super::*;

    #[test]
    fn test_mybox() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // 通过实现 Deref trait，我们可以在智能指针上调用解引用运算符
    }
}

#[cfg(test)]
mod drop_tests {
    use super::*;

    #[test]
    fn test_drop() {
        let x = CustomSmartPoint {
            data: String::from("my stuff"),
        };
        let y = CustomSmartPoint {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPoint created.");
    }
}
