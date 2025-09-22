/*
 * @lc app=leetcode.cn id=3005 lang=rust
 *
 * [3005] 最大频率元素计数
 *
 * https://leetcode.cn/problems/count-elements-with-maximum-frequency/description/
 *
 * algorithms
 * Easy (70.82%)
 * Likes:    20
 * Dislikes: 0
 * Total Accepted:    21.3K
 * Total Submissions: 28.2K
 * Testcase Example:  '[1,2,2,3,1,4]'
 *
 * 给你一个由 正整数 组成的数组 nums 。
 *
 * 返回数组 nums 中所有具有 最大 频率的元素的 总频率 。
 *
 * 元素的 频率 是指该元素在数组中出现的次数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,2,3,1,4]
 * 输出：4
 * 解释：元素 1 和 2 的频率为 2 ，是数组中的最大频率。
 * 因此具有最大频率的元素在数组中的数量是 4 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,3,4,5]
 * 输出：5
 * 解释：数组中的所有元素的频率都为 1 ，是最大频率。
 * 因此具有最大频率的元素在数组中的数量是 5 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * 1 <= nums[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        // 迭代器fold累计: 统计频数Map
        let freq: HashMap<i32, i32> = nums.into_iter().fold(HashMap::new(), |mut m, x| {
            *m.entry(x).or_insert(0) += 1;
            m
        });

        let max_f: i32 = *freq.values().max().unwrap_or(&0); // 找到最大频数值
        freq.values().filter(|&&f| f == max_f).sum() // 该频数求和(不用排序)
    }
}
// @lc code=end
