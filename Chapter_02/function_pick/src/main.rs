/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-28 22:55:26
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-28 23:01:21
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 提取函数以消除重复代码
 */

fn find_max(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_max(&number_list);
    println!("The largest number is {}", result);
}
