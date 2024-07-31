/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 21:25:13
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 22:32:16
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 使用 Mutex<T> 来在多线程中共享数据
 */

// $$ Mutex 是 Mutual Exclusion(互斥)的缩写
// 互斥是一种并发原语，它只允许一个线程访问数据，以确保数据不会同时被多个线程访问
// 想要访问互斥数据，线程首先需要通过获取互斥锁(lock)来表明其意图

// $$ Mutex 的两条规则
// 1. 在访问互斥数据之前，线程必须先通过获取互斥锁
// 2. 在访问互斥数据之后，线程必须通过释放互斥锁

// $$ Mutex<T> 的 API
// 1. new()     创建一个新的互斥锁
// 2. lock()    获取锁
// 3. unlock()  释放锁

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         // lock() 方法返回一个叫做 MutexGuard 的智能指针
//         // MutexGuard 实现了 Deref 来指向其内部数据
//         // MutexGuard 在离开作用域时会自动释放锁
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}", m);
// }

// Arc 与 Rc 类似
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 克隆 Arc 以便在线程间共享所有权
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
