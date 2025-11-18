/*
 * @lc app=leetcode.cn id=1437 lang=rust
 *
 * [1437] 是否所有 1 都至少相隔 k 个元素
 *
 * https://leetcode.cn/problems/check-if-all-1s-are-at-least-length-k-places-away/description/
 *
 * algorithms
 * Easy (56.25%)
 * Likes:    47
 * Dislikes: 0
 * Total Accepted:    31.2K
 * Total Submissions: 53.1K
 * Testcase Example:  '[1,0,0,0,1,0,0,1]\n2'
 *
 * 给你一个由若干 0 和 1 组成的数组 nums 以及整数 k。如果所有 1 都至少相隔 k 个元素，则返回 true ；否则，返回 false
 * 。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：nums = [1,0,0,0,1,0,0,1], k = 2
 * 输出：true
 * 解释：每个 1 都至少相隔 2 个元素。
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入：nums = [1,0,0,1,0,1], k = 2
 * 输出：false
 * 解释：第二个 1 和第三个 1 之间只隔了 1 个元素。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * 0 <= k <= nums.length
 * nums[i] 的值为 0 或 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        // 任意两个1之间距离 >= k
        let num_enum = nums.into_iter().enumerate(); // 迭代器转(idx, val)元组结构; 注意: into / into_iter 区别
        let mut last1idx: i32 = -k - 1; // 上一次1的位置, 用于计算距离; 初始化为一个安全的距离(避免遇到1直接false)
        for (i, x) in num_enum {
            match x {
                0 => continue,
                1 => {
                    let dist: i32 = (i as i32) - last1idx - 1; // 用下标计算距离, 需要额外减1
                    let is_safe: bool = dist >= k;
                    if !is_safe {
                        return false;
                    }
                    // 说明上一轮距离安全, 更新下标位置, 切换到下一轮
                    last1idx = i as i32;
                }
                _ => unreachable!(),
            };
        }
        true
    }
}
// @lc code=end
