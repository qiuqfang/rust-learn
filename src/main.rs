#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn create(name: String, age: u8) -> Person {
        Person { name, age }
    }
    fn say(&self) {
        println!("我的名字叫{}, 今年{}岁", self.name, self.age);
    }
}

fn main() {
    let str = "Hello, world!";
    println!("str {}", str);
    let str1 = str.replace("world", "Rust");
    println!("str1 {}", str1);
    let idx = str.find("world");
    println!("idx {}", idx.unwrap());

    let str2 = &str1[0..5];
    println!("str2 {}", str2);

    let p1 = Person {
        name: String::from("小邱"),
        age: 12,
    };
    println!("我的名字叫{}, 今年{}岁", p1.name, p1.age);
    println!("{:?}", p1);
    println!("{:#?}", p1);
    p1.say();

    let p2 = Person::create(String::from("小明"), 12);
    println!("我的名字叫{}, 今年{}岁", p2.name, p2.age);
    println!("{:?}", p2);
    println!("{:#?}", p2);
    p2.say();
}
