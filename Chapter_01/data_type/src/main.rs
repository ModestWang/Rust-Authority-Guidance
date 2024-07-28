/*
* @FilePath: main.rs
* @Author: Modest Wang 1598593280@qq.com
* @Date: 2024-02-11 23:29:07
* @LastEditors:
* @LastEditTime: 2024-02-16 16:11:55
* 2024 by Modest Wang, All Rights Reserved.
* @Descripttion: data_type

*/

fn main() {
    // Integer
    let int_example: i32 = 10;
    println!("int_example = {}", int_example);

    // Float
    let float_example: f32 = 10.5;
    println!("float_example = {}", float_example);

    // Boolean
    let bool_example: bool = true;
    println!("bool_example = {}", bool_example);

    // Character
    let char_example_1: char = 'A';
    let char_example_2: char = '😊';
    println!("char_example_1 = {}", char_example_1);
    println!("char_example_2 = {}", char_example_2);

    // Array
    let array_example_1: [i32; 5] = [1, 2, 3, 4, 5];
    let array_example_2: [i32; 5] = [0; 5];
    println!("array_example_1 = {:?}", array_example_1);
    println!("array_example_2 = {:?}", array_example_2);

    // Tuple
    let tuple_example: (i32, f32, bool) = (10, 20.5, true);
    println!("tuple_example = {:?}", tuple_example);

    // Struct
    struct Person {
        name: String,
        age: i32,
        is_adult: bool,
    }
    let p: Person = Person {
        name: String::from("John Doe"),
        age: 30,
        is_adult: true,
    };
    println!(
        "p.name = {0},p.age = {1},p.is_adult = {2}",
        p.name, p.age, p.is_adult
    );
}
