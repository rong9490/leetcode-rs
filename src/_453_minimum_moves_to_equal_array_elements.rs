/*
 * @lc app=leetcode.cn id=453 lang=rust
 *
 * [453] 最小操作次数使数组元素相等
 *
 * https://leetcode.cn/problems/minimum-moves-to-equal-array-elements/description/
 *
 * algorithms
 * Medium (61.39%)
 * Likes:    602
 * Dislikes: 0
 * Total Accepted:    92.1K
 * Total Submissions: 150K
 * Testcase Example:  '[1,2,3]'
 *
 * 给你一个长度为 n 的整数数组，每次操作将会使 n - 1 个元素增加 1 。返回让数组所有元素相等的最小操作次数。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3]
 * 输出：3
 * 解释：
 * 只需要3次操作（注意每次操作会增加两个元素的值）：
 * [1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1,1,1]
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * n == nums.length
 * 1 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * 答案保证符合 32-bit 整数
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {

        // max * len - sum = 
        // nums.iter().sum::<i32>() - nums.iter().min().unwrap() * nums.len() as i32

        // 两者等价, 但是迭代器能够更好的防止sum溢出
        let min_num: i32 = *nums.iter().min().unwrap();
        nums.into_iter().fold(0, |count, num| count + num - min_num)

        // let n: usize = nums.len(); // 元素个数
        // let size: usize = n - 1; // 每次操作的个数
        // let mut max: i32 = i32::MIN; // 初始最小值
        // let mut max: &mut i32 = &mut nums[0]; // 初始最小值
        // let mut max: i32 = nums[0]; // 初始最小值
    }
}
// @lc code=end

