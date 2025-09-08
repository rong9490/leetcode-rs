/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根
 *
 * https://leetcode.cn/problems/sqrtx/description/
 *
 * algorithms
 * Easy (38.81%)
 * Likes:    1705
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 2.8M
 * Testcase Example:  '4'
 *
 * 给你一个非负整数 x ，计算并返回 x 的 算术平方根 。
 *
 * 由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。
 *
 * 注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：x = 4
 * 输出：2
 *
 *
 * 示例 2：
 *
 *
 * 输入：x = 8
 * 输出：2
 * 解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= x <= 2^31 - 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // 0 <= x <= 2^31 - 1 给的数据范围已经是i32, 相乘后必须用i64才不会溢出
        // 二分比较, 缩小范围
        let x: i64 = x as i64;
        let (mut result, mut low, mut high) = (0, 0, x);

        while low <= high {
            let mid: i64 = low + (high - low) / 2;
            let pow_val: i64 = mid * mid;
            // 3种大小比较情况
            match pow_val {
                sqr if sqr < x => {
                    result = mid; // 平方数满足, 但是不一定是最大, 再次尝试
                    low = mid + 1;
                }
                sqr if sqr > x => {
                    high = mid - 1; // 过大了, 不满足
                }
                sqr if sqr == x => {
                    result = mid; // 恰好等于, 可以直接返回
                    break;
                }
                _ => unreachable!(),
            }
        }
        result as i32 // 满足的最大值
    }
}
// @lc code=end
