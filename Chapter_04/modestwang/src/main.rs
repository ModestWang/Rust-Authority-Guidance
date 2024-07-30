/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-30 22:41:53
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-30 22:53:55
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// 使用 cargo login 命令登录 crates.io
// 使用 cargo publish 命令发布 crate
// 使用 cargo yunk 命令撤销发布 crate
// 注意: crates.io 不支持删除已发布的 crate

use modestwang::modestwang_do;

fn main() {
    modestwang_do::hello();
}
