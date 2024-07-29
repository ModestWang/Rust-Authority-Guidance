/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-27 20:15:32
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-28 19:44:26
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// Rust 没有像其他语言那样的异常处理机制。

// $$Rust 的错误分为两类：
// - 可恢复错误（recoverable）：例如文件未找到。
// - 不可恢复错误（unrecoverable）：例如尝试访问超过数组结尾的位置。

// $$对于可恢复错误: Rust 使用 Result<T, E> 类型来处理。
// Result<T, E> 是一个枚举类型，有两个成员：
// - Ok(T): 表示操作成功，T 是成功时返回的值。
// - Err(E): 表示操作失败，E 是失败时返回的错误类型。

// $$对于不可恢复错误: Rust 使用 panic! 宏来处理。
// Panic 默认情况下会发生以下操作：
// - 程序展开调用栈: 沿着调用栈向上查找，清理每个函数的数据。
// - 立即中止调用栈: 不进行清理操作，直接退出程序，稍后由操作系统清理。

// 如果想让二进制文件更小，可以在 Cargo.toml 中添加如下配置：
// [profile.release]
// panic = 'abort'
// 这样 panic 会直接退出程序，不会展开调用栈。

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];
}
