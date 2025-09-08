/*
 * @lc app=leetcode.cn id=1317 lang=rust
 *
 * [1317] 将整数转换为两个无零整数的和
 *
 * https://leetcode.cn/problems/convert-integer-to-the-sum-of-two-no-zero-integers/description/
 *
 * algorithms
 * Easy (62.94%)
 * Likes:    54
 * Dislikes: 0
 * Total Accepted:    28.7K
 * Total Submissions: 45K
 * Testcase Example:  '2'
 *
 * 「无零整数」是十进制表示中 不含任何 0 的正整数。
 * 
 * 给你一个整数 n，请你返回一个 由两个整数组成的列表 [a, b]，满足：
 * 
 * 
 * a 和 b 都是无零整数
 * a + b = n
 * 
 * 
 * 题目数据保证至少有一个有效的解决方案。
 * 
 * 如果存在多个有效解决方案，你可以返回其中任意一个。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 2
 * 输出：[1,1]
 * 解释：a = 1, b = 1。a + b = n 并且 a 和 b 的十进制表示形式都不包含任何 0。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 11
 * 输出：[2,9]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：n = 10000
 * 输出：[1,9999]
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：n = 69
 * 输出：[1,68]
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：n = 1010
 * 输出：[11,999]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 <= n <= 10^4
 * 
 * 
 */

struct Solution;

// @lc code=start
use rand::Rng;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        // 从1开始, 无上限...枚举判断每一个值是否满足
        for a in 1.. {
            let rest: i32 = n - a;
            // 同时不含有0, 则返回
            if !a.to_string().contains('0') && !rest.to_string().contains('0') {
                return vec![a, rest];
            }
        }
        unreachable!()

        // 随机数
        // let mut rng = rand::thread_rng();
        // loop {
        //     let a: i32 = rng.gen_range(1..n);
        //     let rest: i32 = n - a;
        //                 if !a.to_string().contains('0') && !rest.to_string().contains('0') {
        //         return vec![a, rest];
        //     }
        // }
        // unreachable!()
    }
}
// @lc code=end

