/*
 * @lc app=leetcode.cn id=3289 lang=rust
 *
 * [3289] 数字小镇中的捣蛋鬼
 *
 * https://leetcode.cn/problems/the-two-sneaky-numbers-of-digitville/description/
 *
 * algorithms
 * Easy (86.66%)
 * Likes:    22
 * Dislikes: 0
 * Total Accepted:    19.6K
 * Total Submissions: 22.5K
 * Testcase Example:  '[0,1,1,0]'
 *
 * 数字小镇 Digitville 中，存在一个数字列表 nums，其中包含从 0 到 n - 1 的整数。每个数字本应 只出现一次，然而，有 两个
 * 顽皮的数字额外多出现了一次，使得列表变得比正常情况下更长。
 *
 * 为了恢复 Digitville 的和平，作为小镇中的名侦探，请你找出这两个顽皮的数字。
 *
 * 返回一个长度为 2 的数组，包含这两个数字（顺序任意）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入： nums = [0,1,1,0]
 *
 * 输出： [0,1]
 *
 * 解释：
 *
 * 数字 0 和 1 分别在数组中出现了两次。
 *
 *
 * 示例 2：
 *
 *
 * 输入： nums = [0,3,2,1,3,2]
 *
 * 输出： [2,3]
 *
 * 解释:
 *
 * 数字 2 和 3 分别在数组中出现了两次。
 *
 *
 * 示例 3：
 *
 *
 * 输入： nums = [7,1,5,4,3,4,6,0,9,5,8,2]
 *
 * 输出： [4,5]
 *
 * 解释:
 *
 * 数字 4 和 5 分别在数组中出现了两次。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= n <= 100
 * nums.length == n + 2
 * 0 <= nums[i] < n
 * 输入保证 nums 中 恰好 包含两个重复的元素。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        // 用数组来统计出现的次数
        let mut arr: [i32; 100] = [0; 100];
        nums.iter().for_each(|v| {
            let v: usize = *v as usize;
            arr[v] += 1;
        });

        // 过滤出现次数大于1的下标, 并返回
        let arr: Vec<i32> = arr
            .iter()
            .enumerate()
            .filter(|(_i, val)| **val > 1)
            .map(|(i, _val)| i as i32)
            .collect::<Vec<i32>>();
        arr
    }
}
// @lc code=end
