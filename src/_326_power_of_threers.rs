/*
 * @lc app=leetcode.cn id=326 lang=rust
 *
 * [326] 3 的幂
 *
 * https://leetcode.cn/problems/power-of-three/description/
 *
 * algorithms
 * Easy (52.65%)
 * Likes:    359
 * Dislikes: 0
 * Total Accepted:    279.5K
 * Total Submissions: 530.9K
 * Testcase Example:  '27'
 *
 * 给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。
 *
 * 整数 n 是 3 的幂次方需满足：存在整数 x 使得 n == 3^x
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 27
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 0
 * 输出：false
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 9
 * 输出：true
 *
 *
 * 示例 4：
 *
 *
 * 输入：n = 45
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * -2^31 <= n <= 2^31 - 1
 *
 *
 *
 *
 * 进阶：你能不使用循环或者递归来完成本题吗？
 *
 */

struct Solution;

// @lc code=start

fn is_exact_nums(n: i32) -> bool {
    match n {
        1 => true,
        3 => true,
        9 => true,
        27 => true,
        81 => true,
        243 => true,
        729 => true,
        2187 => true,
        6561 => true,
        19683 => true,
        59049 => true,
        177147 => true,
        531441 => true,
        1594323 => true,
        4782969 => true,
        14348907 => true,
        43046721 => true,
        129140163 => true,
        387420489 => true,
        1162261467 => true,
        _ => false,
    }
}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // is_exact_nums(n)
        // n > 0 && 1162261467 % n == 0

        let mut m: i32 = n;
        if n == 1 {
            return true;
        } else if n <= 2 {
            return false;
        }

        // 循环除尽
        while m > 1 {
            if m % 3 != 0 {
                return false;
            }
            m = m / 3;
        }
        true
    }
}
// @lc code=end
