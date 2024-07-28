// Rust代码组织

// 模块系统：
// Package(包): Cargo的一个功能，允许你构建、测试和分享crate
// Crate(单元包): 一个二进制或库
// Module(模块): 一个文件
// Path(路径): 用来指定模块、结构体、函数等

// Package和Crate
// 一个包(package)包含一个Cargo.toml文件，它描述了如何构建一个或多个crate
// 只能有0-1个library crate，可以有多个binary crate
// 必须包含至少一个crate(library 或 binary)

// Cargo的惯例
// src/main.rs是binary crate的入口
// src/lib.rs是library crate的入口
// Crate Root：Rust编译器从这个文件开始，然后编译整个crate

// Crate
// 将相关功能组合到一个作用域内，便于在项目间共享和重用
// 如：rand crate提供了随机数生成器，可以在多个项目中使用

// Module
// 在一个crate中，可以将代码分组到多个模块中
// 增加可读性，方便重用
// 控制项目的私有性

// 建立Module
// 使用mod关键字声明一个模块
// 模块内部的代码默认是私有的
// 使用pub关键字将模块内部的代码公开
// 可嵌套模块

// Rust中的所有Item(函数、方法、结构体、枚举、模块和常量)默认是私有的
// 父级模块不能访问子模块的私有Item
// 子模块可以访问所有父模块的Item

// use关键字
// use关键字将路径引入作用域，与python的import类似

// use的习惯用法
// 函数：将函数的父级模块引入作用域(指定到父级)：use crate::front_of_house::hosting;
// struct、enum、其他：指定完整路径(指定到本身)：use crate::back_of_house::Breakfast::toast;
// 重命名：use std::io::Result as IoResult;

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write
use std::io::{self, Write};

fn main() {}
