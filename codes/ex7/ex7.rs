fn main() {
    // 使用小括号和逗号创建元组
    // 元组中可以有很多不同类型的元素，但是对于每一个位置，类型都是确定的，无法修改类型
    let mix_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true, "Hello", (1,2));
    
    let vector: (i32, i32) = (1, 2); // 你可以这样标注元组的类型
    // 访问元组的元素需要使用元组索引
    // 从零开始
    println!("x: {}, y: {}", vector.0, vector.1);

    // #1
    // 可以使用 let 将元组中的内容逐个赋值给别的变量
    let (x, y) = vector;
    println!("x: {}, y: {}", x, y);
    let (first, .., hello, last) = mix_tuple;
    println!("first: {}, hello: {}", first, hello);

    // #2
    // 下面这句是错误的，我们已经帮你注释掉了
    // println!("short tuple {}", vector);
    // 元组可以打印，但是必须使用这种形式
    println!("short tuple {:?}", vector);
    // 下面这句是错误的，我们已经帮你注释掉了
    // println!("too long tuple {:?}", mix_tuple);

    // 一个元素也可以是元组
    // 不过注意必须要有一个逗号
    // 否则会被认为是一个表达式 #3
    let single_integer: i32 = (1);
    let single_tuple: (i32,) = (1,);
    println!("integer: {}", single_integer);
    // !error! 下面这句是错误的！
    // 上面一句可以打印，说明其类型为整型
    // println!("single_tuple: {}", single_tuple);

    // !error! 下面这句是错误的！
    // 我们定义的 vector 变量是不可变的（之后会说明）
    // vector.1 = 1;

    let mut mut_vector = (1, 2i32); // 这是一个可变的 vector
    mut_vector.0 = 1;
    // !error! 下面这句是错误的！
    // 虽然你可以改变元组中的值，但是你不能改变它的类型 #4
    // mut_vector.1 = 1.0;
    println!("mut_vector: {:?}", mut_vector);
}