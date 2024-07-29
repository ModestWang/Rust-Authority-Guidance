/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-07-29 10:12:32
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-07-29 17:28:10
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Trait（特质特征）是一种定义共享行为的方法，类似于其他语言中的接口。
 */
use trait_::NewsArticle;
use trait_::Summary;
use trait_::Wechat;

fn main() {
    let news_article = NewsArticle {
        headline: String::from("The headline of the news"),
        location: String::from("Anhui, China"),
        author: String::from("ModestWang"),
        content: String::from("The content of the news"),
    };
    println!("{}", news_article.summarize());

    let wechat = Wechat {
        username: String::from("ModestWang"),
        content: String::from("The content of the wechat"),
    };
    println!("{}", wechat.summarize());
}

// $$ 实现 trait 的约束
// 实现 trait 的类型或者 trait 本身是在本地 crate 里定义的。
// 例如，我们可以实现标准库中定义的 Display trait 来打印自定义类型的实例。
// 但是，我们不能为外部类型实现外部 trait。
// 这是因为 trait 和类型必须在本地 crate 中定义。

// 这个限制被称为相干性（coherence）或者 孤儿规则(orphan rule)。
// 这个规则确保了其他人的代码不会破坏你的代码，反之亦然。
// 也就是说，两个 crate 不会无意中实现相同的 trait 逻辑，这可能会导致冲突。
