/*
 * @lc app=leetcode.cn id=2485 lang=rust
 *
 * [2485] 找出中枢整数
 *
 * https://leetcode.cn/problems/find-the-pivot-integer/description/
 *
 * algorithms
 * Easy (79.50%)
 * Likes:    72
 * Dislikes: 0
 * Total Accepted:    38.7K
 * Total Submissions: 49.1K
 * Testcase Example:  '8'
 *
 * 给你一个正整数 n ，找出满足下述条件的 中枢整数 x ：
 *
 *
 * 1 和 x 之间的所有元素之和等于 x 和 n 之间所有元素之和。
 *
 *
 * 返回中枢整数 x 。如果不存在中枢整数，则返回 -1 。题目保证对于给定的输入，至多存在一个中枢整数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 8
 * 输出：6
 * 解释：6 是中枢整数，因为 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：1
 * 解释：1 是中枢整数，因为 1 = 1 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 4
 * 输出：-1
 * 解释：可以证明不存在满足题目要求的整数。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        // 从 1 --> x --> n 前后两端区间之和相等
        // let range = 1..=n;
        // let x: Option<i32> = range.into_iter().find(|&x| {
        //     尾项 - 首项) * 个数 / 2
        //     let left: i32 = x * (x + 1) / 2;
        //     let right: i32 = (n * (n + 1) / 2) - (x * (x - 1) / 2);
        //     left == right
        // });
        // let x: i32 = x.unwrap_or(-1); // Option转为默认值
        // x

        // 直接求总和
        let total: i32 = n * (n + 1) / 2;
        // 数学推导公式: x ** 2 = n * (n + 1)
        let x: i32 = (total as f64).sqrt() as i32;
        if x * x == total {
            x
        } else {
            -1
        }
    }
}
// @lc code=end
