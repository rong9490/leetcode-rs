/*
 * @lc app=leetcode.cn id=1413 lang=rust
 *
 * [1413] 逐步求和得到正数的最小值
 *
 * https://leetcode.cn/problems/minimum-value-to-get-positive-step-by-step-sum/description/
 *
 * algorithms
 * Easy (73.36%)
 * Likes:    127
 * Dislikes: 0
 * Total Accepted:    53.6K
 * Total Submissions: 73.1K
 * Testcase Example:  '[-3,2,-3,4,2]'
 *
 * 给你一个整数数组 nums 。你可以选定任意的 正数 startValue 作为初始值。
 *
 * 你需要从左到右遍历 nums 数组，并将 startValue 依次累加上 nums 数组中的值。
 *
 * 请你在确保累加和始终大于等于 1 的前提下，选出一个最小的 正数 作为 startValue 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-3,2,-3,4,2]
 * 输出：5
 * 解释：如果你选择 startValue = 4，在第三次累加时，和小于 1 。
 * ⁠               累加求和
 * startValue = 4 | startValue = 5 | nums
 * (4 -3 ) = 1  | (5 -3 ) = 2    |  -3
 * (1 +2 ) = 3  | (2 +2 ) = 4    |   2
 * (3 -3 ) = 0  | (4 -3 ) = 1    |  -3
 * (0 +4 ) = 4  | (1 +4 ) = 5    |   4
 * (4 +2 ) = 6  | (5 +2 ) = 7    |   2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2]
 * 输出：1
 * 解释：最小的 startValue 需要是正数。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,-2,-3]
 * 输出：5
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * -100 <= nums[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        // 最小的正整数基底, 累加每一位数字, 保证每次相加结果大于0!
        // 尝试先暴力, 再贪心

        // let mut base: i32 = 0;
        // loop {
        //     base += 1;
        //     let mut flag: bool = true;
        //     let mut sum: i32 = base;
        //     for val in &nums {
        //         sum += *val;
        //         if sum <= 0 {
        //             flag = false;
        //             break
        //         }
        //     }
        //     if flag {
        //         return base;
        //     }
        // }

        // 本质是求前缀和, 然后每项必须大于0
        // let min_prefix = nums
        //     .iter()
        //     .scan(0, |sum, &x| {
        //         *sum += x; // 前缀和每一项, 等于当前值加上前一项累加值
        //         Some(*sum)
        //     })
        //     .min()
        //     .unwrap_or(0);

        // 转为迭代器
        let nums_iter = nums.iter();
        // 使用scan, 必须返回Option<T>, 可选的
        let presum = nums_iter.scan(0, |sum, &x| {
            *sum += x; // 前缀和每一项, 等于当前值加上前一项累加值
            return Some(*sum);
        });
        let min_presum: i32 = presum.min().unwrap_or(0);
        1 - min_presum.min(0)
    }
}
// @lc code=end
