/*
 * @lc app=leetcode.cn id=728 lang=rust
 *
 * [728] 自除数
 *
 * https://leetcode.cn/problems/self-dividing-numbers/description/
 *
 * algorithms
 * Easy (76.16%)
 * Likes:    278
 * Dislikes: 0
 * Total Accepted:    92.7K
 * Total Submissions: 121.8K
 * Testcase Example:  '1\n22'
 *
 * 自除数 是指可以被它包含的每一位数整除的数。
 *
 *
 * 例如，128 是一个 自除数 ，因为 128 % 1 == 0，128 % 2 == 0，128 % 8 == 0。
 *
 *
 * 自除数 不允许包含 0 。
 *
 * 给定两个整数 left 和 right ，返回一个列表，列表的元素是范围 [left, right]（包括两个端点）内所有的 自除数 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：left = 1, right = 22
 * 输出：[1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
 *
 *
 * 示例 2:
 *
 *
 * 输入：left = 47, right = 85
 * 输出：[48,55,66,77]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= left <= right <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_self_dividing_numbers(num_ref: &i32) -> bool {
        let num: i32 = *num_ref;
        // 判断每一位数都自整除: 先拆分各自的字符, 转为数字
        num.to_string()
            .chars()
            .map(|chr| chr.to_digit(10).unwrap() as i32)
            .all(|dight| dight != 0 && num % dight == 0)
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        // 模拟区间每个值验证
        (left..=right)
            .into_iter()
            .filter(Solution::is_self_dividing_numbers) // 可以简写方法调用
            .collect()
    }
}
// @lc code=end
