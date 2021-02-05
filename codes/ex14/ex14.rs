/** 从键盘获得输入，返回解析的正整数值
 *  每一次输入使用回车确认
 *  在内部处理了输入失败或无法解析则会报错并且要求重新输入。
 *  @returns u64 返回解析后的整数
 */
fn get_uint64() -> u64 {
    println!("input an positive integer: ");
    loop { // 使用 loop 持续输入
        let mut input = String::new();
        if let Ok(_c_len) = std::io::stdin().read_line(&mut input) {
            input.pop();
            if let Ok(number) = input.parse::<u64>() {
                // 利用返回语句跳出 loop 循环
                return number;
            } else {
                // 解析失败
                println!("Parse failed, please input a positive integer");
            }
        } else {
            println!("Input failed! Please input again!");
        }
    }
}
fn main() {
    let upper_bound: u64 = get_uint64();
    // 处理特殊情况
    if upper_bound <= 1{
        // 没有质数可以打印
    } else {
        let mut n = 2; // 从 2 开始枚举
        while n <= upper_bound {
            // 使用穷举法判断质数
            let mut i = 2;
            let mut flag = true;
            let prime_upper_bound = (n as f64).powf(0.5);

            if prime_upper_bound >= 2.0 {
                while i as f64 <= prime_upper_bound {
                    if n % i == 0 {
                        flag = false;
                        break;
                    }
                    i += 1;
                }
            }
            if flag {
                print!("{}, ", n)
            }
            n += 1;
        }
    }
    println!("\nThe end.");
}