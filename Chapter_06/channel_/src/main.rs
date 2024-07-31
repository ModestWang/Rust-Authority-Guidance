/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 20:10:11
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 21:14:36
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 使用 Channel 进行线程间通信
 */

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        //println!("val = {}", val); //! val的所有权已经转移给了接收者，这里会报错
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
