/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-31 14:46:46
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-31 15:03:00
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Message 1.");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Message 2.");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Message 3.");
        }
    }
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, messenger: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(messenger));
        }
    }
}

// $$ borrow()      返回 Ref<T> 类型的智能指针
// $$ borrow_mut()  返回 RefMut<T> 类型的智能指针

// $$ RefCell<T> 用于在运行时记录借用规则
// $$ 即：RefCell<T>会记录当前存在多少个活跃的 Ref<T> 和 RefMut<T> 智能指针
