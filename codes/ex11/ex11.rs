fn main() {
    // 若干打印使用的宏
    let format_string: String = format!("Hello, {}", 233);
    print!("{}", format_string);
    println!("Hello World!");
    eprint!("Hello");
    eprintln!("World");

    // 使用数字指定打印的位置
    println!("{0}长，{1}宽。{1}没有{0}长，{0}没有{1}宽。", "扁担", "板凳");
    // 使用变量指定打印的位置
    println!("A hamburger has a structure of: {upper}|{center}|{lower}",
        upper="bread",
        center="meat",
        lower="bread"
    );

    // 格式化参数示意
    println!("{argument:*^+#0width$.4?}", argument=3.1415926, width=10); // 完全体参数示意
    println!("128 == 0b{:b}", 128); // 使用2进制进行打印，并且在前面加上0b前缀
    println!("27 == {:#x}", 27); // 使用16进制打印整数
    println!("{:*^20}", format!("{{Hello,}} {:+#08x}", 233)); // 各种参数的综合运用

    #[derive(Debug)]
    struct Pair(i32, i32);
    println!("{:?}", Pair(2, 3)); // 使用 Debug 进行格式化
    println!("{:#?}", Pair(2, 3)); // 让 Debug 格式化更好看
}