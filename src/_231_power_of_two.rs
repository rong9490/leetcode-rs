/*
 * @lc app=leetcode.cn id=231 lang=rust
 *
 * [231] 2 的幂
 *
 * https://leetcode.cn/problems/power-of-two/description/
 *
 * algorithms
 * Easy (49.99%)
 * Likes:    774
 * Dislikes: 0
 * Total Accepted:    417.8K
 * Total Submissions: 835.7K
 * Testcase Example:  '1'
 *
 * 给你一个整数 n，请你判断该整数是否是 2 的幂次方。如果是，返回 true ；否则，返回 false 。
 *
 * 如果存在一个整数 x 使得 n == 2^x ，则认为 n 是 2 的幂次方。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 1
 * 输出：true
 * 解释：2^0 = 1
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 16
 * 输出：true
 * 解释：2^4 = 16
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 3
 * 输出：false
 *
 *
 *  吗,:'
 * []po7i654wq
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
 * 进阶：你能够不使用循环/递归解决此问题吗？
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // n > 0 && (1 << 30) % n == 0

        // 从二进制及位运算来看到问题:
        // 2的幂二进制必定是: 10000 不定个数的0, 但是不可能有1
        // n - 1 的二进制必定是 01111
        n > 0 && (n & (n - 1) == 0)
    }
}
// @lc code=end
