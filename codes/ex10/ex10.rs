// 一个结构体
struct Meal{
    calorie: i32
}
impl Meal {
    fn calculate_calorie(&self){
        println!("This meal has {} calories.", self.calorie);
    }
}
// 创建枚举类型
// 枚举类型中的类别需要是 struct
enum MyDay {
    Nothing, // 可以是 unit struct
    Sleep(i32), // 可以是 tuple struct
    Meal(Meal), //  #1 注意：你不能直接把一个定义好的结构体放在枚举类型中
    Walk { x: f64, y: f64 }, //可以是一般的结构体
} // 没有分号

// 枚举类也可以拥有自己的函数
impl MyDay {
    // #2 这是一个成员函数。参数代表的是当前的 MyDay 枚举类储存的内容，没有返回值(返回())
    fn discribe(&self) { // self 代表自身，可以认为 &self 代表的是 this 指针
        // #3 match 是一个关键词，可以匹配变量中的内容
        // 你可以把它当作这个 Rust 中的 switch 使用
        match self {
            MyDay::Nothing => println!("I'm doing nothing..."),
            MyDay::Meal(meal) => { print!("I'm having my meal."); meal.calculate_calorie(); },
            // 从元组中获得数据
            MyDay::Sleep(time) => println!("I have slept for {} hours.", time),
            // 从结构体中解构得到数据
            MyDay::Walk{ x, y } => println!("I'm walking to pos: ({}, {})", x, y),
            _ => println!("What is this ?"), // 默认的匹配选项
        }
    }
}

fn main(){
    let nothing = MyDay::Nothing;
    let meal = MyDay::Meal(Meal {calorie: 1000});
    let sleep = MyDay::Sleep(8);
    let walk = MyDay::Walk{x: 12.0, y: 13.0};

    nothing.discribe();
    meal.discribe();
    sleep.discribe();
    walk.discribe();
}