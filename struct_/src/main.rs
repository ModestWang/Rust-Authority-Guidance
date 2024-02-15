#[derive(Debug)] // 开启打印调试信息

// 定义一个struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 实现一个关联函数
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
}

fn main() {
    let user1 = User::new(String::from("user1"), String::from("123456"));

    println!(
        "user1: username: {}, email: {}, sign_in_count: {}, active: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    // 输出struct实例信息有好几种方式：
    // 1.实现 std::fmt::Display trait
    // 2.实现 std::fmt::Debug trait
    // 3.#[derive(Debug)] 开启打印调试信息
    println!("user1: {:?}", user1); // 打印调试信息
    println!("user1: {:#?}", user1); // 格式化输出
}
