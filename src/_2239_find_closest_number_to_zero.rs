/*
 * @lc app=leetcode.cn id=2239 lang=rust
 *
 * [2239] 找到最接近 0 的数字
 *
 * https://leetcode.cn/problems/find-closest-number-to-zero/description/
 *
 * algorithms
 * Easy (59.31%)
 * Likes:    39
 * Dislikes: 0
 * Total Accepted:    33.3K
 * Total Submissions: 56.2K
 * Testcase Example:  '[-4,-2,1,4,8]'
 *
 * 给你一个长度为 n 的整数数组 nums ，请你返回 nums 中最 接近 0 的数字。如果有多个答案，请你返回它们中的 最大值 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：nums = [-4,-2,1,4,8]
 * 输出：1
 * 解释：
 * -4 到 0 的距离为 |-4| = 4 。
 * -2 到 0 的距离为 |-2| = 2 。
 * 1 到 0 的距离为 |1| = 1 。
 * 4 到 0 的距离为 |4| = 4 。
 * 8 到 0 的距离为 |8| = 8 。
 * 所以，数组中距离 0 最近的数字为 1 。
 * 
 * 
 * 示例 2：
 * 
 * 输入：nums = [2,-1,1]
 * 输出：1
 * 解释：1 和 -1 都是距离 0 最近的数字，所以返回较大值 1 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 1000
 * -10^5 <= nums[i] <= 10^5
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let ans: Option<&i32> = nums.iter().min_by_key(|&x| -> (i32, i32) {
            // 额外元组返回个-x, 因为当绝对值相等, 返回更大的
            (x.abs(), -x)
        });
        *ans.unwrap()
    }
}
// @lc code=end

