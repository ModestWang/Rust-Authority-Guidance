use b::adder; // 需要在Cargo.toml 的 [dependencies] 中添加 crate 路径

fn main() {
    println!("{}", adder::add(1, 1));
}
