/*
 * @lc app=leetcode.cn id=2566 lang=rust
 *
 * [2566] 替换一个数字后的最大差值
 *
 * https://leetcode.cn/problems/maximum-difference-by-remapping-a-digit/description/
 *
 * algorithms
 * Easy (70.86%)
 * Likes:    40
 * Dislikes: 0
 * Total Accepted:    19.2K
 * Total Submissions: 27.2K
 * Testcase Example:  '11891'
 *
 * 给你一个整数 num 。你知道 Danny Mittal 会偷偷将 0 到 9 中的一个数字 替换 成另一个数字。
 *
 * 请你返回将 num 中 恰好一个 数字进行替换后，得到的最大值和最小值的差为多少。
 *
 * 注意：
 *
 *
 * 当 Danny 将一个数字 d1 替换成另一个数字 d2 时，Danny 需要将 num 中所有 d1 都替换成 d2 。
 * Danny 可以将一个数字替换成它自己，也就是说 num 可以不变。
 * Danny 可以将数字分别替换成两个不同的数字分别得到最大值和最小值。
 * 替换后得到的数字可以包含前导 0 。
 * Danny Mittal 获得周赛 326 前 10 名，让我们恭喜他。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：num = 11891
 * 输出：99009
 * 解释：
 * 为了得到最大值，我们将数字 1 替换成数字 9 ，得到 99899 。
 * 为了得到最小值，我们将数字 1 替换成数字 0 ，得到 890 。
 * 两个数字的差值为 99009 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：num = 90
 * 输出：99
 * 解释：
 * 可以得到的最大值是 99（将 0 替换成 9），最小值是 0（将 9 替换成 0）。
 * 所以我们得到 99 。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= num <= 10^8
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let num_str: String = num.to_string(); // 先转为字符串
        let mut max: i32 = i32::MIN; // i32负数最小值

        for c in '0'..'9' {
            // 假设把num_str中的c替换成9, 从左开始替换(高位替换); 再比较较大值
            max = max.max(num_str.replace(c, "9").parse::<i32>().unwrap());
        }

        // 最小值: 高位替换为0
        let min = num_str.replace(&num_str[0..1], "0").parse::<i32>().unwrap();

        max - min
    }
}
// @lc code=end
