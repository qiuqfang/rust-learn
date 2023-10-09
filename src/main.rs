fn main() {
    // immutable()
    // changeable()
    // constant()
    data_type_list();
}

fn data_type_list() {
    // 字面量
    let str: &str = "Hello, world!";
    println!("str {}", str);
    let str1: String = str.replace("world", "Rust");
    println!("str1 {}", str1);
    let idx: Option<usize> = str.find("world");
    println!("idx {}", idx.unwrap());

    // 切片
    let str2 = &str1[0..5];
    println!("str2 {}", str2);
}

// fn constant() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
//     // THREE_HOURS_IN_SECONDS = 3;
// }

// fn changeable() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn immutable() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     // x = 6;
//     // println!("The value of x is: {x}");
// }
