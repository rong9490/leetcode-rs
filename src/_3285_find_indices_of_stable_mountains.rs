/*
 * @lc app=leetcode.cn id=3285 lang=rust
 *
 * [3285] 找到稳定山的下标
 *
 * https://leetcode.cn/problems/find-indices-of-stable-mountains/description/
 *
 * algorithms
 * Easy (90.94%)
 * Likes:    16
 * Dislikes: 0
 * Total Accepted:    20.4K
 * Total Submissions: 22.4K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 有 n 座山排成一列，每座山都有一个高度。给你一个整数数组 height ，其中 height[i] 表示第 i 座山的高度，再给你一个整数
 * threshold 。
 * 
 * 对于下标不为 0 的一座山，如果它左侧相邻的山的高度 严格大于 threshold ，那么我们称它是 稳定 的。我们定义下标为 0 的山 不是
 * 稳定的。
 * 
 * 请你返回一个数组，包含所有 稳定 山的下标，你可以以 任意 顺序返回下标数组。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：height = [1,2,3,4,5], threshold = 2
 * 
 * 输出：[3,4]
 * 
 * 解释：
 * 
 * 
 * 下标为 3 的山是稳定的，因为 height[2] == 3 大于 threshold == 2 。
 * 下标为 4 的山是稳定的，因为 height[3] == 4 大于 threshold == 2.
 * 
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：height = [10,1,10,1,10], threshold = 3
 * 
 * 输出：[1,3]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：height = [10,1,10,1,10], threshold = 10
 * 
 * 输出：[]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 <= n == height.length <= 100
 * 1 <= height[i] <= 100
 * 1 <= threshold <= 100
 * 
 * 
 */


struct Solution;

// @lc code=start
use std::ops::Range;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let range: Range<usize> = (1..height.len());
        let filted = range.filter(|&i| {
            height[i - 1] > threshold
        });
        let mapped = filted.map(|i| i as i32); // 转一遍类型
        // 转为容器
        mapped.collect::<Vec<i32>>()
    }
}
// @lc code=end

