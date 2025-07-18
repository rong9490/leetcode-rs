/*
 * @lc app=leetcode.cn id=3065 lang=rust
 *
 * [3065] 超过阈值的最少操作数 I
 *
 * https://leetcode.cn/problems/minimum-operations-to-exceed-threshold-value-i/description/
 *
 * algorithms
 * Easy (89.31%)
 * Likes:    20
 * Dislikes: 0
 * Total Accepted:    29.2K
 * Total Submissions: 32.7K
 * Testcase Example:  '[2,11,10,1,3]\n10'
 *
 * 给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。
 *
 * 一次操作中，你可以删除 nums 中的最小元素。
 *
 * 你需要使数组中的所有元素都大于或等于 k ，请你返回需要的 最少 操作次数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,11,10,1,3], k = 10
 * 输出：3
 * 解释：第一次操作后，nums 变为 [2, 11, 10, 3] 。
 * 第二次操作后，nums 变为 [11, 10, 3] 。
 * 第三次操作后，nums 变为 [11, 10] 。
 * 此时，数组中的所有元素都大于等于 10 ，所以我们停止操作。
 * 使数组中所有元素都大于等于 10 需要的最少操作次数为 3 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,1,2,4,9], k = 1
 * 输出：0
 * 解释：数组中的所有元素都大于等于 1 ，所以不需要对 nums 做任何操作。
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,1,2,4,9], k = 9
 * 输出：4
 * 解释：nums 中只有一个元素大于等于 9 ，所以需要执行 4 次操作。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 50
 * 1 <= nums[i] <= 10^9
 * 1 <= k <= 10^9
 * 输入保证至少有一个满足 nums[i] >= k 的下标 i 存在。
 *
 *
 */

struct Solution {}

// @lc code=start
use std::slice::Iter;
use std::vec::IntoIter;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        // 解法1: 不可变迭代器, 产生了双重引用!  iter + filter
        // let nums_iter: Iter<'_, i32> = nums.iter();
        // let nums_iter_filter = nums_iter.filter(|__val: &&i32| {
        //     let val: i32 = **__val;
        //     return val < k;
        // });
        // let count: usize = nums_iter_filter.count();
        // count as i32

        // 解法2: 可变迭代器
        let nums_iter: IntoIter<i32> = nums.into_iter();
        let nums_iter_filter = nums_iter.filter(|num: &i32| *num < k);
        nums_iter_filter.fold(0, |acc, _| acc + 1)
    }
}
// @lc code=end
