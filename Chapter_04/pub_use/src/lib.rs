/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-30 22:29:17
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-30 22:29:18
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 使用 pub use 导出方便使用的公共 API
 */

//! # Art
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColors;
pub use kinds::SecondaryColors;
pub use utils::mix;

pub mod kinds {
    /// Primary colors are red, yellow, and blue.
    pub enum PrimaryColors {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary colors are orange, green, and purple.
    pub enum SecondaryColors {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two strings with `&` in the middle.
    pub fn mix(s1: &str, s2: &str) -> String {
        format!("{} & {}", s1, s2)
    }
}
