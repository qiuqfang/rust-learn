/**
 * 普通结构体
 * #[derive(Debug)] 用于完整展示结构体
 */
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
impl Person {
    // 关联函数
    fn create(name: String, age: u8) -> Person {
        Person { name, age }
    }
    // 方法
    fn say(&self) {
        println!("我的名字叫{}, 今年{}岁", self.name, self.age);
    }
}

fn main() {
    // 创建实例
    let p1 = Person {
        name: String::from("小邱"),
        age: 12,
    };
    println!("我的名字叫{}, 今年{}岁", p1.name, p1.age);
    p1.say();

    // 通过关联函数创建实例
    let p2 = Person::create(String::from("小明"), 12);
    println!("我的名字叫{}, 今年{}岁", p2.name, p2.age);
    p2.say();

    display_struct(p1, p2);
}

fn display_struct(p1: Person, p2: Person) {
    println!("{:?}", p1);
    println!("{:#?}", p1);
    println!("{:?}", p2);
    println!("{:#?}", p2);
}
