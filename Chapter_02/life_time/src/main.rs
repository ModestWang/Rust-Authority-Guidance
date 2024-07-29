/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 17:27:47
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 19:13:04
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: 生命周期: 引用保持有效的作用域。
 */
// 大多数情况下：生命周期是隐式的，可以通过函数签名中的引用来推断。
// 当引用的生命周期可能以不同的方式相互关联时：必须显式标注生命周期。

fn main() {
    {
        // let r;
        // {
        //     let x = 5;
        //     r = &x;
        // }
        // println!("r: {}", r); //! 错误: r借用的x已经被销毁，r指向的内存已经被释放。

        // $$ Rust 使用借用检查器来检查引用是否有效：
        // 1. 引用的生命周期不能超过其引用的对象。
        // 2. 引用的生命周期不能超过其作用域。

        // 解决方法：将r的生命周期延长到x的作用域。
        // let x = 5;
        // let r = &x;
        // println!("r: {}", r);

        // $$ 生命周期注解
        let string1 = String::from("long string is long");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

// ! 错误：返回值的生命周期未知，无法推断。
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// $$ 生命周期注解：告诉Rust 引用的生命周期应该符合某种特定的模式。
// $$ 生命周期注解语法：&'a T，其中'a是生命周期参数。
//*注意：生命周期的注解不会改变任何引用的生命周期，只是表明引用应该符合特定的生命周期。
// 生命周期 'a 的实际生命周期是函数调用时传入的引用的生命周期的子集。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// $$ 返回引用类型时，需要关注生命周期

// $$ 生命周期省略的三条规则：
// 1. 每个是引用的参数都有自己的生命周期参数。
// 2. 如果<只有一个>输入生命周期参数，那么它被赋予所有的输出生命周期参数。
// 3. 如果方法<有多个>输入生命周期参数，不过其中一个是 &self 或 &mut self，那么self的生命周期被赋予所有的输出生命周期参数。

// 其中，规则1用于输入生命周期参数，规则2和规则3用于输出生命周期参数。
// 如果Rust 编译器根据这三条规则无法推断生命周期 --> 编译器报错（悬垂引用错误）。
// 这些规则适用于 fn定义 和 impl块。

// $$ 'static 是一个特殊的生命周期，它代表整个程序的生命周期（持续时间）。
// 例如，所有的字符串字面值都拥有 'static 生命周期。
