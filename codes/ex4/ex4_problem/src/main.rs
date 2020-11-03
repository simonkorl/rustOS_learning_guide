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

    // 打印tuple
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
  
    // 在{}中使用一定规则就可以达到想要的输出，不需要额外函数对10进制数字进行转换
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
  
    // (12, true, "hello")是一个tuple，不可以直接输出，因此要使用{:?}debug trait
    println!("{:?}", (12, true, "hello"));
    
}