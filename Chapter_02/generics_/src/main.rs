/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-28 23:04:22
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 10:03:39
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 泛型
 */

fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![123.1, 34.3, 600.0, 23.0, 45.0, 67.0, 89.0];
    let result = find_largest(&number_list);
    println!("The largest number is {}", result);
}
