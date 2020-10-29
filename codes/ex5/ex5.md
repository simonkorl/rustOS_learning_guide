# ex5 基本数据类型——标量 (Primitives —— Scalar Types)

Rust 具有非常多的原始数据类型，这些数据类型是用来描述储存在电脑中的变量类型。这次我们说明的是标量类型 Scalar Types

标量类型是 Rust 中最为基本的数据类型，通常也是计算机处理数据的最基本的单元。Rust 当中的标量包括：

1. bool 类型，代表逻辑的正误，有 true 和 false 两个值
2. 整型：分为有符号和无符号两个类型，根据位数有8,16,31,64,128以及指针大小几个种类
3. 浮点型：有32与64位浮点数
4. char 字符型：每一个字符都是 Unicode 字符，占4个字符
5. unit type：单元类型。可以认为是一个没有数据的元素，占用空间为 0。可以安全地进行空元素处理。

```rust
fn main(){
    // 使用 let 语句进行变量绑定
    // 你可以不知道什么是变量绑定，只要抄下来就对了
    
    // 在变量绑定的时候可以使用 ':' 标注类型
    let logical: bool = true;
    println!("1 == 1 is {}", logical);

    // 两种不同的标注方法
    let a_float: f32 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    
    // 如果不标注则会使用默认的类型（由字面量决定） 
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // 整数后面代表储存用的 bit 数
    // 这决定了它的范围
    let int8: i8 = 127; // [-128,127] 8位有符号整型
    let uint8: u8 = 255; // [0, 255] 8位无符号整型
    let int16: i16 = 32767; // [-32768, 32767] 16位有符号整型
    let uint16: u16 = 65535; // [0, 65535] 16位无符号整型

    // error!! 下面的四行代码无法运行
    // 你可以注释掉以下的四行代码
    // 请注意编译器的错误提示，isize 与 usize 会随着机器变化
    let pointer: isize = 2333333333333333333333; // pointer size
    let int64: i64 = 2333333333333333333333;    // i64 与 64位机上的 isize 相同
    let upointer: usize = 6666666666666666666666; // pointer size
    let upointer: u64 = 6666666666666666666666; // u64 与 64位机上的 usize 相同

    // 编译器可以根据上下文推断类型
    // let mut 定义的是可以变的变量，你还不需要知道它的意思
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // error: 下面这段代码有错误！
    // 你不能这么做不是因为它计算的结果不合法
    // 而是它们类型不相同，并不能进行计算（虽然看上去很明显可以）
    println!("1u32 - 1i32 = {}", 1u32 - 1i32);

    // 字符类型
    let en: char = 'x'; // char: 4B
    let zh: char = '中'; // unicode 四个字节可以表示现在的所有字符

    return (); // unit type 是所有没有标注返回值的函数的默认返回类型
}
```

这些基本数据类型最基本的特点是它们占据的空间是固定的，比如说 bool 类型占 1 字节，char 类型占 4 字节，整数和浮点类型与标注的大小一致，unit type 不占据空间。这些数据类型是程序中储存数据的最为基础的单位。

有关数据类型我们不深入进行讲解，因为如果要深入讲解又可以重新写一本书的一章出来了。对于基本数据类型而言，基本上所有的程序设计语言在这方面都大同小异，概念非常接近。Rust 在基本概念上最接近 C 和 C++ 。如果你对整数类型、浮点类型这样的计算机基础概念还不清楚，可以到网络上查找 C 或 C++ 的资料进行学习。

注：因为数据类型是非常基础的计算机概念，如果这方面不熟的话一定要好好巩固以下基础技术。《深入理解计算机系统》这本书中讲解得很详细，可以作为参考。

## 参考资料

- [Rust By Example：Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)

- 《深入理解计算机系统》