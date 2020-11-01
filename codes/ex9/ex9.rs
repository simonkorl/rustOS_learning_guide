// 一般的结构体
struct Vec2D{
    x: f64,
    y: f64
} // 没有分号 #1

struct Unit; // Unit 结构体，和 Unit type 相同，只有名字但是不占空间。有分号

struct Pair(i32, i32); // 元组结构体，使用元组的形式创建结构体。有分号
struct Single(i32); // 元组结构体，这时并不需要写逗号。有分号

#[derive(Debug)] // 写了这句话，下面一句的 struct 才可以进行打印
struct Person<'a> {
    // 别问这个`'a`是做什么用的，先写上 #2
    name: &'a str,
    age: u8
}

struct Rect {
    top_left: Vec2D, // 使用结构体作为参数类型
    bottom_right: Vec2D
}
// 结构体可以有自己的函数 #3
impl Rect {
    // 计算长方形的面积
    fn area(&self) -> f64 {
        // 计算并返回面积
        // 计算机中一般左上角规定为(0,0)，右和下分别是x轴和y轴的正方向
        (self.bottom_right.x - self.top_left.x) * (self.bottom_right.y - self.top_left.y)
    }
}

fn main() {
    // #4
    // 一般结构体的`实例化`(Instantiate)方法
    let point: Vec2D = Vec2D { x: 23.3, y: 66.6};
    // 对结构体中的域(field)进行访问
    println!("point.x: {}, point.y: {}", point.x, point.y);

    // 元组结构体的实例化方法，直接使用名称以及元组的形式
    let pair: Pair = Pair(1, 3);
    // 访问元组结构体的域
    println!("pair.0: {}, pair.1: {}", pair.0, pair.1);
    // 可以类似元组地对变量进行赋值
    let Pair(x, y) = pair;
    println!("pair.0 => x: {}, pair.1 => y: {}", x, y);

    let single = Single(7);
    println!("{:?}", single.0);

    // Unit 结构体的实例化方法
    let _unit = Unit;
    // 如果你让其继承了 Debug trait（也就是写了#[derive(Debug)]） 那么它会打印出来 Unit，也就是声明的struct的名称
    // println!("{:?}", unit);

    let name = "Mike";
    let age = 18;
    // 如果你的变量名和结构体的域名相同，则可以简化
    let person = Person { 
        name, 
        age
    };
    // 使用了 #[derive(Debug)] 的结构体可以使用默认的打印方法 #5
    println!("{:?}", person);
    // 没有实现任何东西的前提下，结构体是不能被打印的
    // 下面这句话不能运行，我们帮你注释掉了
    // println!("{:?}", point);
    
    // !error! 下面这句话是错误的，你需要略作修改
    let top_left = Vec2D {x: 0, y: 0};
    let bottom_right = Vec2D {x: 13.0, y: 15.5};
    let rectangle = Rect { top_left, bottom_right};
    println!("the area of rectangle: {}", rectangle.area()); // 调用结构体的函数
}