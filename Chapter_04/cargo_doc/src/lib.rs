/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-30 22:19:27
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-30 22:21:00
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// $$ #Examples 章节中的代码会被 cargo doc 识别为文档测试
// $$ 即，使用 cargo test 命令会执行这些代码

//! # cargo_doc
//! It is uesd to test cargo doc.
//! Created by ModestWang.
/// Adds two to the number given.
/// # Examples
/// ```
/// let five = cargo_doc::add_two(3);
/// assert_eq!(5, five);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}
