/*
 * @FilePath: intergration_test.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 20:56:41
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 21:00:55
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 集成测试
 */

// $$ 集成测试：在 tests 目录下创建测试文件
// cargo test --test intergration_test
// 每个测试文件都是一个独立的 crate

mod common;

#[test]
fn it_add_two() {
    assert_eq!(test_::add(2, 2), 4);
}

// $$ 说明
// - 如果项目是二进制 crate，只含有 src/main.rs 而没有 src/lib.rs
// -- 那么不能在 tests 目录下创建集成测试文件
// -- 无法把 main.rs 的函数导入到 tests 目录下的测试文件中
// -- 因为只有 library crate 才能暴露函数供其他 crate 使用
