/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 *
 * https://leetcode.cn/problems/move-zeroes/description/
 *
 * algorithms
 * Easy (64.01%)
 * Likes:    2727
 * Dislikes: 0
 * Total Accepted:    2M
 * Total Submissions: 3.1M
 * Testcase Example:  '[0,1,0,3,12]'
 *
 * 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
 *
 * 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: nums = [0,1,0,3,12]
 * 输出: [1,3,12,0,0]
 *
 *
 * 示例 2:
 *
 *
 * 输入: nums = [0]
 * 输出: [0]
 *
 *
 *
 * 提示:
 *
 *
 *
 * 1 <= nums.length <= 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 *
 *
 *
 *
 * 进阶：你能尽量减少完成的操作次数吗？
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // 可以使用双指针, 逐步移动交换的位置
        let size: usize = nums.len();
        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < size {
            // 将所有不为0的值往前移动(交换); 如果为0则不交换, 后续直接覆盖
            if nums[j] != 0 {
                let tmp: i32 = nums[i];
                nums[i] = nums[j];
                nums[j] = tmp;
                i += 1;
            }
            j += 1;
        }
    }
}
// @lc code=end
