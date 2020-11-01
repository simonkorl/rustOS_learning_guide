# struct

## 结构体(structure)

结构体(structure，简称为 struct)是 Rust 中最为重要也是最为灵活的一种自定义数据类型。Rust 的结构体与很多语言的结构体或是类的概念并不一致，这使得这个概念更加难以理解，不妨直接从代码开始看起会比较好。

## 代码

```rust
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
```

## 代码说明

### #1

Rust 提供三种不同的构造结构体的方法：

1. braced struct: 和 C 语言相似，使用大括号创建结构体的声明，不过没有分号进行结尾（因为并没有进行实例化）
2. 元组结构体（tuple struct）:从各个角度上都像是元组（tuple）。可以认为是带有名字的元组。在声明的时候需要使用分号结尾
3. 单元结构体（unit struct）:和单元类型（unit type）有些相似，不占用空间，可以认为只有名字。在“泛型”中很有用。

要生成一个结构体需要进行两个步骤：“声明”(declare)结构体，然后将其“实例化”(instantiate)。当你“声明”结构体时，你告诉了编译器这个结构体中有哪些内容，具有哪些属性，有哪些函数，形成一个抽象的概念。当你“实例化”结构体的时候，结构体中的数据部分被绑定了特定的具体数据，在内存中真正占据了空间，此时结构体的生命周期正式开始。

### #2

在代码中有一个很奇怪的`'a`标志，这个标志称为“生命周期”注释。顾名思义，“生命周期”代表了一个变量“能活多久”。

“生命周期”和“所有权”机制保证了 Rust 代码中的引用安全。我们会在一个单独的章节来说明这件事。现在只需要知道如果要在结构体中存放引用，那么标注生命周期是必要的——你可以尝试删除`'a`的标注，查看编译器的报错。

### #3

对于 struct 可以绑定自己的函数这件事情如果用 C 的角度来想可能有一点奇怪，不过如果从类的角度上来考虑 struct 未来就会感觉有一点奇怪。不妨从这个角度上来考虑：

struct 是一个储存数据的容器，就比如说一个饭盒；然后你可以在饭盒外面绑定一些工具，比如说筷子、勺子什么的。Rust 的结构体就是这样一种可以绑定函数的储存数据的结构，和其他编程语言的类的概念有一些近似，不过也有一点不同。如果真的要类比的话，Rust 的struct 可能更加像一个严格的 javascript 的对象(Object)类型。

你现在完全不需要会写 struct 的“成员函数”，你只要明白有这样一种形式就可以了。

### #4

结构体的实例方法根据结构体的类型不同，会有不同的感受。比如说一般结构体的实例化就非常像 javascript 的对象创建，元组结构体的实例化就非常像元组，单元结构体的实例化就是直接赋值。

结构体的域的访问也和它本身的类型有关系，比如元组结构体就需要使用元组访问域的方法访问其中的元素，一般结构体的元素访问利用域名和成员描述符`.`。这些操作都是比较符合直觉的。

### #5

我们在 ex11 中会详细说明与打印相关的更加高级的东西，在这里我们稍微提一下。

`println!("{:?}", something)`可以使用 Debug 的方式打印一个类型。所有的标准库的类型都默认继承了 Debug trait，也就是它们都可以进行 Debug 打印，比如说 Range, Vector, String（我们未来会看到）。而 struct 没有默认过这个 trait ,所以一般情况下不能直接使用`"{:?}"`进行打印。

如果你希望简单地调试 struct 中的内容，可以如同代码中写的一样在 struct 声明的上一行写上`#[derive(Debug)]`，这样就可以利用`{:?}`进行输出。不过这样的输出也许不是你想要的，看起来可能也不是很好看。如果你需要自定义一个结构体的输出方式的话可以参看 ex11 的内容。

## 本节总结

在这一节中，我们针对自定义数据类型`struct`进行了初步介绍。可以发现即使是简单的初步介绍我们要需要介绍很多内容，有一些内容可能还是现在没办法完全解释清楚的。你可以先掌握其中的核心概念，让自己可以看懂之后的大部分代码，至于更加深入的理解可以在之后参考更加专业的资料进行学习。

你应该对以下内容有所掌握：

1. 三种不同的`struct`的声明以及实例化方法
2. tuple struct 可以使用类似元组的方法进行各种操作

你应该对以下内容有所了解：

1. `struct`可以使用`impl`块定义自身的函数，并且利用实例化后的对象进行调用
2. `struct`可以通过继承`Debug` trait 进行 Debug 打印
3. 如果要在`struct`中加入引用，则需要标注“生命周期”

## 参考资料

- [1] [Rust By Example: structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [2] [Rust 生命周期 | 菜鸟教程](https://www.runoob.com/rust/rust-lifetime.html)
- [3] [doc.rust-lang.org: structures](https://doc.rust-lang.org/stable/reference/types/struct.html)