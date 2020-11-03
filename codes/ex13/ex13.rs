fn main() {
    let x = 2i32; // 这是一个 statement

    let y = {
        x * x // 这是一个 expression。因为没有分号所以它的值能够传递下去
    };

    let z = {
        y + 1; // 这是一个 statement，没有值。因为 expression 有分号，所以它的值无法向下传递
    };

    println!("x: {}", x); // 这些打印宏与分号的组合也是 statement
    println!("y: {}", y);
    println!("z: {}", z);
} // 大括号组成的部分是一个 block，具有值。