/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 16:54:45
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 16:54:51
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// 使用 #[macro_export] 宏来导出宏
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}


use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // 代码
}
