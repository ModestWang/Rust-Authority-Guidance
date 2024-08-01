/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 00:44:45
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 00:52:21
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// $$ 封装: 通过结构体和枚举将数据和行为组合在一起
// 在 Rust 中，默认所有的成员都是私有的，所以需要使用 pub 关键字来使其公有

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// $$ 继承: 通过 trait 对象实现继承
// - 代码复用
// - 多态
