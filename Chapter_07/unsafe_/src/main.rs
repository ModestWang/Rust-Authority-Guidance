/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-08-01 01:12:59
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-08-01 15:40:45
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */

// $$ unsafe rust 中可执行的动作
// 1. 解引用裸指针(原始指针)
// 2. 调用 unsafe 函数或方法
// 3. 访问或修改可变静态变量
// 4. 实现 unsafe trait
// 5. 使用 extern 函数调用其他语言的函数

unsafe fn dangerous() {
    println!("dangerous");
}

// $$ 从其他语言调用 Rust 函数
// 需要在函数签名上加上 extern "C"，这样编译器就会使用 C 调用约定来调用这个函数
// 通过 #[no_mangle] 属性来禁用 Rust 编译器的名称重整
#[no_mangle]
pub extern "C" fn cal_from_c() {
    println!("call from C");
}

fn main() {
    // $$ 解引用裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // $$ 调用 unsafe 函数或方法
    unsafe {
        dangerous();
    }

    // $$ 访问或修改可变静态变量
    // 静态变量是一个在程序的整个生命周期都存在的变量
    // 命名规范是全大写字母和下划线，且必须标注变量的类型
    static HELLO: &str = "Hello, World!";
    println!("name is: {}", HELLO);

    // $$ 实现 unsafe trait
    unsafe trait Foo {
        fn foo(&self);
    }

    struct Bar;

    unsafe impl Foo for Bar {
        fn foo(&self) {
            println!("Bar");
        }
    }

    let bar = Bar;
    bar.foo();

    // $$ 使用 extern 函数调用其他语言的函数
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    let num = -3;
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(num));
    }
}
