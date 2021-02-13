/// 从键盘获得输入，返回解析的正整数值
/// 
/// 每一次输入使用回车确认
/// 
/// 在内部处理了输入失败或无法解析则会报错并且要求重新输入。
/// 
/// @returns u64 返回解析后的整数

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


struct Solution;
impl Solution {
    // #3
    fn solution(n: u64) -> u64 {
        return {
            if n == 1 {
                1 // #1
            } else if n == 2 {
                2
            } else {
                let mut dp = [0u64; 3]; // 使用基本的动态规划的思路
                dp[0] = 1; // 上1节台阶有一种方法
                dp[1] = 2; // 上2节台阶有两种方法
                for _i in 3..=n { // #2
                    dp[2] = dp[0] + dp[1];
                    dp[0] = dp[1];
                    dp[1] = dp[2];
                }
                dp[2]
            }
        }
    }
}
fn main() {
    println!("{}", Solution::solution(get_uint64()));
}