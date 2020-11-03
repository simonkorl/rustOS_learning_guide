fn main() {
    // 为一个“变量”绑定对应的值
    let variable = "Hello World";
    println!("I said: \"{}\"", variable);
    // “变量”默认是“不可变”(immutabale)
    // !error! 下面这一行是错误的
    variable = "Nya nya nya";
    // 但是你可以靠着 shadowing 来进行重新绑定
    let variable = "Nya nya nya"; // 你需要使用 let 关键字进行 shadowing
    println!("I said: \"{}\"", variable);

    // 定义一个“可变”(mutable)的“变量”
    let mut mutable_variable = 0;
    println!("before mutation: {}", mutable_variable);
    mutable_variable = 1;
    println!("after mutation: {}", mutable_variable);
    
    // 没有被使用的变量应该使用 _ 作为前缀
    let _unused_variable = 2333;
    
    // shadowing 和作用域
    let variable = 1; // 请注意：我们改变了 variable 绑定的数据类型
    println!("outer variable: {}", variable);
    let mut variable = 2; // 将声明过的 variable 进行 shadowing
    println!("outer variable shadowing: {}", variable);
    {
        // 此时我们依然可以访问外层定义的可变 variable 变量
        println!("inner access outer variable: {}", variable);
        variable = 12; // 此时修改的是外层定义的 variable 变量
        println!("inner change outer variable: {}", variable);
        // 在内侧定义一个名称全新的变量
        let another_variable = 'a';
        println!("inner another_variable: {}", another_variable);
        // 在内侧重新绑定一个*名称相同*的变量 variable，但是它和外层的 variable 已经不是同一个变量
        // 这并不会导致外层的变量消失
        let mut variable = 20;
        println!("inner variable: {}", variable);
        variable = 21; // 对重新绑定的变量进行更改
        println!("inner changed variable: {}", variable);
        let variable = variable; // freeze，使用一个定义过的可变变量赋值给同名的不可变变量
        println!("inner freezed variable: {}", variable);
        // !error! 下面这句是错误的。variable 最后一次绑定时是不可变的。
        variable = 16;
    }
    // !error! 下面这句是错误的。inner_variable 已经离开作用域并释放
    println!("out access inner variable: {}", inner_variable);
    println!("outer variable is changed inside: {}", variable); // 可以发现外层的变量已经被修改

    // 可以先声明变量之后赋值，不过这并不推荐
    let binding;
    {
        let value = "Hello World";

        binding = value;
    }
    println!("binding is: {}", binding);
    
    let forget_to_bind;
    // !error! 下面这一行是错误的
    println!("{}", forget_to_bind);
    forget_to_bind = "Oh, I forgot to bind.";
    println!("{}", forget_to_bind);
    
}