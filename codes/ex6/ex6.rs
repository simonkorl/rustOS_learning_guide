fn main() {
    // 字面量表示了计算机可以直接读懂的数据
    // 这个时候请主要注意等号右边的那些量
    let integer = 1024; // 整数，默认为 i32
    let interger_with_underscore = 1_000_000_000u32; // 整数，可以使用下划线分开。如果你不使用默认类型，你需要告诉编译器
    
    // 整数可以使用不同的进制表示
    let bin = 0b10_000_000_000; // 二进制
    let oct = 0o2_000; // 八进制
    let dec = 1024; // 十进制
    let hex = 0x400; // 十六进制

    // 打印出来默认都是十进制，如果希望有不同的打印方法，则可以查看文档![formatting](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html)
    println!("four integers: bin: {}, oct: {}, dec: {}, hex: {}", bin, oct, dec, hex);
   
    let float = 1.0; // 浮点数，默认为 f64
    let float_with_underscore = 0.000_1f32; // 浮点数，可以使用下划线分开。可以通过标注不使用默认类型

    let characters: char = 'a'; // 字符char，使用单引号表示

    let boolean: bool = false; // bool 类型，只有 true 和 false 两个值

    // !注：下面这一行没有错误，但是它会阻止程序继续运行
    // 这个语句代表在程序运行中的某个要求，这里要求两个数值相等
    // 这不是一个错误，而是一个运行时的判断
    assert_eq!(true, boolean); // 判断是否相同

    let unit = (); // unit 类型，不占空间，也没有什么意义
    let another_unit = ();
    assert_eq!(unit, another_unit); // 两者是相同的

    let string: &str = "Hello World"; // 字符串 str，可以认为是一个 Unicode的序列。使用双引号表示
    println!("{}!", string);

    // 运算符
    // Rust 的运算符和类 C 语言的运算符基本一致

    // 算术运算符
    println!("1 + 1 = {}", 1i32 + 1i32);
    println!("1 - 2 = {}", 1i32 - 2i32);
    println!("2 * 2 = {}", 2 * 2);
    println!("3 / 2 = {}", 3 / 2);
    println!("10 % 3 = {}", 10 % 3);

    // error: 以下的两行无法通过编译，请注意它们的报错
    // 注：在第一行被注释之前，第二行不会报错
    println!("2u - 1i = {}", 2u32 - 1i32); // 除非特殊情况，否则类型不同的数据无法进行运算
    println!("1u - 2u = {}", 1u32 - 2u32); // 编译器会检查整数是否会溢出，这在 Rust 中被禁止

    // 逻辑运算符
    // 与、或、非，与类 C 语言相同
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    
    // 位运算符
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 打印4位用0填充的二进制数，打印方式了解即可。需要的时候可以查询文档
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 以16进制整数打印

    // 位运算与二元逻辑
    // 位运算可以应用于 bool 类型上
    println!("true AND true ? {}", true & true);
    println!("true OR false ? {}", true | false);
    println!("true NOR true? {}", true ^ true);
    let result: bool = true & true; // 运算结果为 bool
}