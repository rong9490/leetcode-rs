/*
 * @lc app=leetcode.cn id=3318 lang=rust
 *
 * [3318] 计算子数组的 x-sum I
 *
 * https://leetcode.cn/problems/find-x-sum-of-all-k-long-subarrays-i/description/
 *
 * algorithms
 * Easy (66.61%)
 * Likes:    20
 * Dislikes: 0
 * Total Accepted:    10.8K
 * Total Submissions: 15.1K
 * Testcase Example:  '[1,1,2,2,3,4,2,3]\n6\n2'
 *
 * 给你一个由 n 个整数组成的数组 nums，以及两个整数 k 和 x。
 *
 * 数组的 x-sum 计算按照以下步骤进行：
 *
 *
 * 统计数组中所有元素的出现次数。
 * 仅保留出现次数最多的前 x 个元素的每次出现。如果两个元素的出现次数相同，则数值 较大 的元素被认为出现次数更多。
 * 计算结果数组的和。
 *
 *
 * 注意，如果数组中的不同元素少于 x 个，则其 x-sum 是数组的元素总和。
 *
 * 返回一个长度为 n - k + 1 的整数数组 answer，其中 answer[i] 是 子数组 nums[i..i + k - 1] 的
 * x-sum。
 *
 * 子数组 是数组内的一个连续 非空 的元素序列。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
 *
 * 输出：[6,10,12]
 *
 * 解释：
 *
 *
 * 对于子数组 [1, 1, 2, 2, 3, 4]，只保留元素 1 和 2。因此，answer[0] = 1 + 1 + 2 + 2。
 * 对于子数组 [1, 2, 2, 3, 4, 2]，只保留元素 2 和 4。因此，answer[1] = 2 + 2 + 2 + 4。注意 4
 * 被保留是因为其数值大于出现其他出现次数相同的元素（3 和 1）。
 * 对于子数组 [2, 2, 3, 4, 2, 3]，只保留元素 2 和 3。因此，answer[2] = 2 + 2 + 2 + 3 + 3。
 *
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,8,7,8,7,5], k = 2, x = 2
 *
 * 输出：[11,15,15,15,12]
 *
 * 解释：
 *
 * 由于 k == x，answer[i] 等于子数组 nums[i..i + k - 1] 的总和。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n == nums.length <= 50
 * 1 <= nums[i] <= 50
 * 1 <= x <= k <= nums.length
 *
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n: usize = nums.len(); // 总长度
        let k: usize = k as usize; // 子数组窗口长度
        let x: usize = x as usize; // 统计频数最多的前x位求和
        let mut ans: Vec<i32> = Vec::with_capacity(n - k + 1); // 确定长度为 n - k + 1

        // 一轮循环产生一个数
        for win in nums.windows(k) {
            // 统计当前子数组窗口内, 所有的频数 --> 后续优化为一个共享复用Map
            let freq: HashMap<i32, i32> = win.iter().fold(HashMap::new(), |mut acc, &v| {
                *acc.entry(v).or_insert(0) += 1;
                acc
            });

            // 转为容器, 并排序获取前x位
            let mut freq_vec: Vec<(i32, i32)> = freq.into_iter().collect();
            // 排序: 取频数更大的, 频数相等取值更大
            freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
            // 取前x位take(x), 相同的值乘于个数, 再求总和
            let x_sum: i32 = freq_vec.iter().take(x).map(|(v, c)| v * c).sum();
            ans.push(x_sum);
        }
        ans
    }
}
// @lc code=end
