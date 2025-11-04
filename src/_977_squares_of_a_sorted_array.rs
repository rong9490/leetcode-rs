/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 *
 * https://leetcode.cn/problems/squares-of-a-sorted-array/description/
 *
 * algorithms
 * Easy (68.91%)
 * Likes:    1138
 * Dislikes: 0
 * Total Accepted:    898.9K
 * Total Submissions: 1.3M
 * Testcase Example:  '[-4,-1,0,3,10]'
 *
 * 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-4,-1,0,3,10]
 * 输出：[0,1,9,16,100]
 * 解释：平方后，数组变为 [16,1,0,9,100]
 * 排序后，数组变为 [0,1,9,16,100]
 *
 * 示例 2：
 *
 *
 * 输入：nums = [-7,-3,2,3,11]
 * 输出：[4,9,9,49,121]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums 已按 非递减顺序 排序
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 请你设计时间复杂度为 O(n) 的算法解决本问题
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // 简单做法, 直接排序 -> 时间复杂度为 O(nlogn)
        // let mut ans: Vec<i32> = nums.iter().map(|&n| n * n).collect();
        // ans.sort();
        // ans

        // 双指针: 遍历两遍
        let n: usize = nums.len();
        let mut negative: i32 = -1; // usize不能为负数

        // 找到最大的负数的下标 --> 分界线, 由中间向两端推进
        for i in 0..n {
            if nums[i] < 0 {
                negative = i as i32;
            } else {
                break;
            }
        }

        let mut ans: Vec<i32> = Vec::new();
        // 以最大的负数下标作为分界线, 分别向两端推进, 产出的平方就是非递减的
        let (mut i, mut j): (i32, i32) = (negative, negative + 1);
        while i >= 0 || j < n as i32 {
            let i_usize: usize = i as usize;
            let j_usize: usize = j as usize;

            // 没有剩余负数项
            if i == -1 {
                ans.push(nums[j_usize] * nums[j_usize]);
                j += 1;
                continue;
            }
            // 没有剩余正数项
            if j_usize == n {
                ans.push(nums[i as usize] * nums[i as usize]);
                i -= 1;
                continue;
            }
            let power_left: i32 = nums[i_usize] * nums[i_usize];
            let power_right: i32 = nums[j_usize] * nums[j_usize];
            // 左侧更小
            if power_left < power_right {
                ans.push(power_left);
                i -= 1;
                continue;
            }
            // 右侧更小
            ans.push(power_right);
            j += 1;
        }
        ans
    }
}
// @lc code=end
