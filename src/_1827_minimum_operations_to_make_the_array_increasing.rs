/*
 * @lc app=leetcode.cn id=1827 lang=rust
 *
 * [1827] 最少操作使数组递增
 *
 * https://leetcode.cn/problems/minimum-operations-to-make-the-array-increasing/description/
 *
 * algorithms
 * Easy (81.32%)
 * Likes:    78
 * Dislikes: 0
 * Total Accepted:    47.2K
 * Total Submissions: 58K
 * Testcase Example:  '[1,1,1]'
 *
 * 给你一个整数数组 nums （下标从 0 开始）。每一次操作中，你可以选择数组中一个元素，并将它增加 1 。
 *
 *
 * 比方说，如果 nums = [1,2,3] ，你可以选择增加 nums[1] 得到 nums = [1,3,3] 。
 *
 *
 * 请你返回使 nums 严格递增 的 最少 操作次数。
 *
 * 我们称数组 nums 是 严格递增的 ，当它满足对于所有的 0 <= i < nums.length - 1 都有 nums[i] <
 * nums[i+1] 。一个长度为 1 的数组是严格递增的一种特殊情况。
 *
 *
 *
 * 示例 1：
 *
 * 输入：nums = [1,1,1]
 * 输出：3
 * 解释：你可以进行如下操作：
 * 1) 增加 nums[2] ，数组变为 [1,1,2] 。
 * 2) 增加 nums[1] ，数组变为 [1,2,2] 。
 * 3) 增加 nums[2] ，数组变为 [1,2,3] 。
 *
 *
 * 示例 2：
 *
 * 输入：nums = [1,5,2,4,1]
 * 输出：14
 *
 *
 * 示例 3：
 *
 * 输入：nums = [8]
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 5000
 * 1 <= nums[i] <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        // 目标是"严格递增", 相邻两位作差
        let mut nums: Vec<i32> = nums;
        let range = 0..nums.len() - 1; // 取n, n+1位
        range.fold(0, |acc, i| {
            let left: i32 = nums[i];
            let right: i32 = nums[i + 1];
            // 变为严格递增的最小差值
            let diff: i32 = if left < right {
                0
            } else {
                left - right + 1
            };

            if diff > 0 {
                // 变更, 为前者+1
                nums[i + 1] = left + 1;
            }
            acc + diff
        })
    }
}
// @lc code=end
