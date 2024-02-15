// 在 Rust 中，enum（枚举）是一种可以表示多种可能类型的数据类型。
// 每个枚举成员都可以有不同的类型和数据。这使得枚举非常灵活。

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 方法体
        println!("called");
    }
}

fn main() {
    let a = Message::Quit;
    let b = Message::Move { x: 1, y: 2 };
    let c = Message::Write(String::from("hello"));
    let d = Message::ChangeColor(255, 255, 255);
    a.call();
}
