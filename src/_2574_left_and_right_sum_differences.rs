/*
 * @lc app=leetcode.cn id=2574 lang=rust
 *
 * [2574] 左右元素和的差值
 *
 * https://leetcode.cn/problems/left-and-right-sum-differences/description/
 *
 * algorithms
 * Easy (84.01%)
 * Likes:    27
 * Dislikes: 0
 * Total Accepted:    19.7K
 * Total Submissions: 23.5K
 * Testcase Example:  '[10,4,8,3]'
 *
 * 给你一个下标从 0 开始的长度为 n 的整数数组 nums。
 *
 * 定义两个数组 leftSum 和 rightSum，其中：
 *
 *
 * leftSum[i] 是数组 nums 中下标 i 左侧元素之和。如果不存在对应的元素，leftSum[i] = 0 。
 * rightSum[i] 是数组 nums 中下标 i 右侧元素之和。如果不存在对应的元素，rightSum[i] = 0 。
 *
 *
 * 返回长度为 n 数组 answer，其中 answer[i] = |leftSum[i] - rightSum[i]|。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [10,4,8,3]
 * 输出：[15,1,11,22]
 * 解释：数组 leftSum 为 [0,10,14,22] 且数组 rightSum 为 [15,11,3,0] 。
 * 数组 answer 为 [|0 - 15|,|10 - 11|,|14 - 3|,|22 - 0|] = [15,1,11,22] 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1]
 * 输出：[0]
 * 解释：数组 leftSum 为 [0] 且数组 rightSum 为 [0] 。
 * 数组 answer 为 [|0 - 0|] = [0] 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 1000
 * 1 <= nums[i] <= 10^5
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        // 先暴力完整计算偏差O(n^2) --> 前缀和/后缀和 优化
        // let size: usize = nums.len();
        // let mut ans: Vec<i32> = vec![0; size];
        // for i in 0..size {
        //     // 两端求和, 求diff
        //     let left_sum: i32 = nums[0..i].iter().sum();
        //     let right_sum: i32 = nums[i + 1..].iter().sum();
        //     ans[i] = (left_sum - right_sum).abs();
        // }
        // ans

        let size: usize = nums.len();
        let mut prefix: Vec<i32> = vec![0; size + 1];
        let mut suffix: Vec<i32> = vec![0; size + 1];

        // 计算前缀和: prefix[i] = sum(nums[0..i])
        for i in 0..size {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        // 计算后缀和: suffix[i] = sum(nums[i..size])
        for i in (0..size).rev() {
            suffix[i] = suffix[i + 1] + nums[i];
        }
        let mut ans = vec![0; size];
        for i in 0..size {
            let left_sum = prefix[i]; // nums[0..i]，即i左边所有元素
            let right_sum = suffix[i + 1]; // nums[i+1..n], 即i右边所有元素
            ans[i] = (left_sum - right_sum).abs();
        }

        ans
    }
}
// @lc code=end
