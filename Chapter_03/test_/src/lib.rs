/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 19:56:39
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 20:45:02
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 编写测试
 */

// $$ 使用 cargo test 命令运行测试

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

// $$ 单元测试：使用 #[cfg(test)] 标记测试模块
#[cfg(test)]
mod tests {
    use super::*;

    // $$ 使用 #[test] 标记测试函数
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // $$ 测试失败：发生 panic
    #[test]
    fn test_panic() {
        panic!("Make this test fail");
    }

    // $$ 使用 should_panic 标记测试函数
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn test_should_panic() {
        Guess::new(200);
    }

    // $$ 使用 ignore 标记测试函数
    #[test]
    #[ignore]
    fn test_ignore() {
        assert_eq!(add(2, 2), 4);
    }
}

// $$ assert！宏:
// - 如果表达式为 true，则继续执行
// - 如果表达式为 false，则 panic

// $$ assert_eq！宏 和 assert_ne！宏 与 assert！宏类似，但是它们会比较两个值
// - 如果为 false，则 panic, 并打印出两个值的内容

// $$ assert! 宏的第二个参数（将传递给 format! 宏）可以自定义信息（可选）
// assert!(result == 4, "Expected 4, got {}", result);

// $$ should_panic 标记
// - 测试函数 panic --> 测试通过
// - 测试函数不 panic --> 测试失败
// 可以使用 expect 参数来指定 panic 时的错误信息

// $$ 无需 panic，可使用 Result<T, E> 的测试
// - 返回 Ok(T) --> 测试通过
// - 返回 Err(E) --> 测试失败
// 注意：不要在使用 Result<T, E> 的测试中使用 #[should_panic] 标记

// $$ 关于 cargo test 的使用
// - cargo test 运行所有测试
// -- 默认行为：
// --- 并行运行测试
// --- 默认行为：所有测试
// --- 默认行为：捕获测试输出（隐藏测试输出）

// $$ 针对 cargo test 的命令行参数：放在 cargo test 之后
// - cargo test test_name 运行特定测试
// - cargo test test_name1 test_name2 运行多个测试
// - cargo test test_name* 运行所有名称匹配的测试

// $$ 针对 cargo test 生成的可执行文件的命令行参数：放在 -- 之后
// - cargo test -- --show-output 显示测试输出
// - cargo test -- --test-threads=1 指定线程数（并行和连续执行）
// - cargo test -- --help 查看帮助信息

// $$ ignore 标记 的测试默认不运行，使用 cargo test --ignored 运行

// $$ Rust 允许对私有函数进行测试
