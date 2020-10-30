# ex6 字面量(Literals)

## 字面量说明

字面量如果就从“字面”意义上来讲，就是“所见即所得”。你所输入的变量既是你能理解的，也是计算机可以理解并且直接转化为数据存储下来的。这包括了整数、浮点数、字符、字符串等你可以直接在程序中表示出来的量。

不过注意，这和变量还不是一个概念，比如说一个变量绑定语句`let x = 1;`，从语法上来讲，等号右边的便是字面量，告诉计算机具体要绑定的变量的值是多少，左边的则是变量名，告诉计算机你要把值附到哪一个变量上。

字面量代表了计算机中实际储存的一些基本的值，一般来讲字面量都是可以和基本数据类型进行绑定的。

你可以参考代码进行理解，也可以查找相关的资料进行进一步的了解。

## 运算符(operators)

运算符一般代表一个或是两个量的运算操作，最常见的是算数运算符，比如说'+','-','*','/'，代表算术运算的加减乘除（除法在整数上做处理会抛弃余数）。

除此之外还有：逻辑运算符，用于进行逻辑运算；位运算符，将一个数作为二进制对于每个位进行运算。

有关运算符实际上最为重要的内容是它们之间的先后处理顺序，但是在这一章中我们可能无法进行展现。我们在这里提供一个参考链接，需要的同学也可以在网络上找到更多的资料（可以参考 C 或 C++ 的运算符介绍）。

基本上的原则就是：

1. 有括号先运算括号
2. 一元运算符高于二元运算符
3. 算术运算中间满足乘除高于加减
4. 算术运算高于位运算
5. 位运算高于逻辑运算
6. 同级别中：与 > 异或 > 或
7. 赋值语句仅高于流程控制相关语句

注：实际上运算符的优先级很多还是和直觉相同的，出了错再记住就可以了

[Expression Reference](https://doc.rust-lang.org/reference/expressions.html#expression-precedence)

## 代码说明

```rust
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

    let characters = 'a'; // 字符，使用单引号表示

    let boolean = false; // bool 类型，只有 true 和 false 两个值

    // !注：下面这一行没有错误，但是它会阻止程序继续运行
    // 这个语句代表在程序运行中的某个要求，这里要求两个数值相等
    // 这不是一个错误，而是一个运行时的判断
    assert_eq!(true, boolean); // 判断是否相同

    let unit = (); // unit 类型，不占空间，也没有什么意义
    let another_unit = ();
    assert_eq!(unit, another_unit); // 两者是相同的

    let string = "Hello World"; // 字符串，使用双引号表示，可以认为是一个 Unicode的序列。
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
```

请注意程序中那些具有后缀的字面量。在 ex5 中我们说明了可以用两种方法标注变量的数据类型。实际上它们有一定的区别：使用类似`let x: type = ...;`的方法标注的变量，数据类型绑定在了变量 x 上，编译器会判断你赋给变量的值是否与你标注的相同；使用`let x = 10i8`的方法标注则是在字面量上说明这个字面量的具体类型（因为整型和浮点型有很多个不同的大小，不注明的话编译器只能默认它们是默认大小），编译器会根据字面量的类型推断变量的类型。

关于代码中的`assert_eq!()`，可以发现它也由 ! 结尾，这说明它也是一个宏。这个宏用于断言(assert)，一般用于在程序中判断某个值是不是期望的值（`assert_eq!`判断两个值是否相同，不相同则报错）。如果发现程序的运行过程可能出了错误，则 assert 语句会发出警告并终止程序。但是它检查值的时机是在程序运行的时候，所以哪怕一句看上去明显不正确的断言`assert_eq!(true, false)`也不会导致编译器报错。

[Rust By Example Literals and operators](https://doc.rust-lang.org/rust-by-example/primitives/literals.html)