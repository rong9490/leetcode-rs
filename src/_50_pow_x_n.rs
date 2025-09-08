/*
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * [50] Pow(x, n)
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            1 => x, // 原数
            0 => 1.0, // 0次幂
            -1 => 1.0 / x, // 倒数
            n if n % 2 == 0 => Self::my_pow(x * x, n / 2), // 偶数次幂, 折半递归, 先处理偶数
            n if n < 0 => 1.0 / x * Self::my_pow(x * x, n / 2), // 负数次幂
            n if n % 2 == 1 => x * Self::my_pow(x * x, n / 2), // 奇数次幂,
            _ => unreachable!(),
        }
    }
}
// @lc code=end

