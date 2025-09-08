/*
 * @lc app=leetcode.cn id=268 lang=rust
 *
 * [268] 丢失的数字
 *
 * https://leetcode.cn/problems/missing-number/description/
 *
 * algorithms
 * Easy (67.99%)
 * Likes:    887
 * Dislikes: 0
 * Total Accepted:    400.2K
 * Total Submissions: 588.7K
 * Testcase Example:  '[3,0,1]'
 *
 * 给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [3,0,1]
 *
 * 输出：2
 *
 * 解释：n = 3，因为有 3 个数字，所以所有的数字都在范围 [0,3] 内。2 是丢失的数字，因为它没有出现在 nums 中。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,1]
 *
 * 输出：2
 *
 * 解释：n = 2，因为有 2 个数字，所以所有的数字都在范围 [0,2] 内。2 是丢失的数字，因为它没有出现在 nums 中。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [9,6,4,2,3,5,7,0,1]
 *
 * 输出：8
 *
 * 解释：n = 9，因为有 9 个数字，所以所有的数字都在范围 [0,9] 内。8 是丢失的数字，因为它没有出现在 nums 中。
 *
 *
 * 提示：
 *
 *
 * n == nums.length
 * 1 <= n <= 10^4
 * 0 <= nums[i] <= n
 * nums 中的所有数字都 独一无二
 *
 *
 *
 *
 * 进阶：你能否实现线性时间复杂度、仅使用额外常数空间的算法解决此问题?
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // x ^ y ^ y == x 异或两次会相互抵消 --> 区分 出现一次 / 出现两次
        // 迭代器fold 累计(reduce)
        // 先产生range -> 再chain收尾相连成迭代器(double数组) -> fold累计异或
        let size: i32 = nums.len() as i32;
        let double_nums_iter = (0..=size).chain(nums.into_iter());
        let signal_num: i32 = double_nums_iter.fold(0, |acc, cur| acc ^ cur);
        signal_num
    }
}
// @lc code=end
