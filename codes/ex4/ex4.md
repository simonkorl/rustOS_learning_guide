# ex4 : 打印！打印！打印！

从这一章我们开始真正地编写代码！而最简单且直观的方式就是将感兴趣的一切打印出来。本书的模式是类似于“笨方法”系列，希望你能先原原本本地照抄一遍每一节的代码部分，然后阅读代码说明加深理解，最后可以基于此前的知识完成练习题，cargo test全部通过就说明正确完成了！

## 代码
```rust
fn main() {
    // 使用println!宏将字符串输出到终端，带有换行
    println!("Hello from the print.rs file");
  
    // 使用{}进行格式化输出，类比C语言的printf
    println!("{} is from {}", "Brad", "Mass");
  
    // 可以给后面的参数们编个号，这些编号可以在格式化字符串中调换顺序
    println!(
      "{0} is from {1} and {0} likes to {2}",
      "Brad", "Mass", "code"
    );
  
    // 带有命名的参数
    println!(
      "{name} likes to play {activity}",
      name = "John",
      activity = "Baseball"
    );

    // 参数可以是表达式
    println!("10 + 10 = {}", 10 + 10);

    // 参数可以是基本类型变量
    let name = "Brad";
    let age = 37;
    println!("My name is {} and I am {}", name, age);

    // 打印常量
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // 打印tuple，注意下标访问方式不同于寻常
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
  
    // 在{}中使用一定规则就可以达到想要的输出，不需要额外函数对10进制数字进行转换
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
  
    // (12, true, "hello")是一个tuple，不可以直接输出，因此要使用{:?}debug trait
    println!("{:?}", (12, true, "hello"));
    
}
```


## 代码说明

代码基本的含义行内注释已经解释得比较清楚了，这里不再赘述。

值得着重一提的是，Rust处理print的方式和别的语言有较大的不同。Rust的println!是一种宏，定义在`std::fmt`中，而格式化输出则是通过各种`trait`来决定的。常用的就是：
+ `{}`普通输出，可以输出基本类型，无需考虑类型转换问题，但不能输出tuple、array这些
+ `{:?}`调试输出，可以输出tuple、array等

熟练使用各种printing的trait，可以轻易地实现很多功能。

