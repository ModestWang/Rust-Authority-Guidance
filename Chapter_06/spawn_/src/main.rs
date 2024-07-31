/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 19:48:29
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 19:50:59
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: std::thread::spawn 创建线程
 */

use std::thread;
use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("number {} from the spawned thread", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap(); // 等待线程结束
// }

// 注意：当主线程结束时，所有线程都会被杀死。

fn main() {
    let v = vec![1, 2, 3];

    // 使用move关键字强制获取v的所有权,防止在主线程中被释放导致错误。
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
