/*
 * @lc app=leetcode.cn id=3370 lang=rust
 *
 * [3370] 仅含置位位的最小整数
 *
 * https://leetcode.cn/problems/smallest-number-with-all-set-bits/description/
 *
 * algorithms
 * Easy (81.67%)
 * Likes:    20
 * Dislikes: 0
 * Total Accepted:    20.9K
 * Total Submissions: 25.1K
 * Testcase Example:  '5'
 *
 * 给你一个正整数 n。
 * 
 * 返回 大于等于 n 且二进制表示仅包含 置位 位的 最小 整数 x 。
 * 
 * 置位 位指的是二进制表示中值为 1 的位。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入： n = 5
 * 
 * 输出： 7
 * 
 * 解释：
 * 
 * 7 的二进制表示是 "111"。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入： n = 10
 * 
 * 输出： 15
 * 
 * 解释：
 * 
 * 15 的二进制表示是 "1111"。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入： n = 3
 * 
 * 输出： 3
 * 
 * 解释：
 * 
 * 3 的二进制表示是 "11"。
 * 
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
    pub fn smallest_number(n: i32) -> i32 {
        // 理解"置数", 也就是二进制全是1
        // 1 + 3 + 7 + 15 + 31
        // 从1开始, 左移一位(等价于乘于2) + 1
        // 直到 ret >= n, 此时ret是满足条件最小数
        let grow = |x: i32| -> i32 {
            (x << 1) + 1
        };
        let mut ret: i32 = 1;
        while ret < n {
            ret = grow(ret);
        }
        ret
    }
}
// @lc code=end

