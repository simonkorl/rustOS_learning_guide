// #1
// 我们定义了另一个函数，你只需要注意切片的类型就可以
fn analyze_slice(slice: &[i32]) {
    println!("The whole slice: {:?}", slice);
    println!("The length of this slice: {}", slice.len());
}
// 如果有若干个函数，那么会从 main 函数开始运行
fn main() {
    // 数组：具有固定的大小，可以放在栈上
    let array: [i32; 5] = [0, 1, 2, 3, 4];
    let n_array: [i32; 10]; // 数组也可以稍后初始化，不过极不推荐
    n_array = [0; 10]; // 初始化为一个全是零的序列

    // 使用下标访问数组元素
    println!("array[0]: {}", array[0]); // 数组下标从 0 开始
    println!("array[4]: {}", array[4]); // 最后一个元素，下标是长度 - 1

    // `len`函数可以返回数组的长度
    println!("array size: {}", array.len());

    // 打印数组，使用{:?}
    println!("n_array: {:?}", n_array);

    // 切片：不知道有多大，不可以直接放在栈上
    // 下面这行不正确，我们替你注释掉了
    // let slice: [i32] = array[0..2]; // [i32]是一个切片的类型，因为没有长度所以不能放在栈上
    // 0..2代表一个区间，左闭右开 #2
    let slice: &[i32] = &array[0..2]; // &[i32]在栈上的是一个切片的引用，大小为两个 usize
    println!("{:?}", slice);

    // 数组整体上可以被认为是一个切片
    // 如果需要进行转化的话，数组可以自动被当作切片进行“借用”(borrow)
    // 下面这行是错误的，我们替你注释掉了
    // analyze_slice(array);
    analyze_slice(&array); // 如果想要进行“借用”，那么必须使用 & 符号。细节留到后面
    
    // 借用一部分数组作为切片
    analyze_slice(&array[0..=3]); // 0..=3代表一个区间，两边均是闭区间
    
    // 数组访问不允许越界
    let pos = 5; 
    // !error! 下面这句话不正确
    println!("array[5]: {}", array[pos]); // 长度为5的数组能访问的最大下标为4

    // 编译器有的时候可以帮你解决一些问题，不过你可以瞒过去
    for i in 0..6 { //#3
        // 编译器没办法找到这个地方的问题
        // 不过 Rust 已经想了办法阻止越界访问导致的结果
        println!("array[{}]: {}", i, array[i]);
    }
}