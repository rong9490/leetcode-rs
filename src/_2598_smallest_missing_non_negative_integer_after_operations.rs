/*
 * @lc app=leetcode.cn id=2598 lang=rust
 *
 * [2598] 执行操作后的最大 MEX
 *
 * https://leetcode.cn/problems/smallest-missing-non-negative-integer-after-operations/description/
 *
 * algorithms
 * Medium (40.72%)
 * Likes:    40
 * Dislikes: 0
 * Total Accepted:    15.9K
 * Total Submissions: 33.2K
 * Testcase Example:  '[1,-10,7,13,6,8]\n5'
 *
 * 给你一个下标从 0 开始的整数数组 nums 和一个整数 value 。
 *
 * 在一步操作中，你可以对 nums 中的任一元素加上或减去 value 。
 *
 *
 * 例如，如果 nums = [1,2,3] 且 value = 2 ，你可以选择 nums[0] 减去 value ，得到 nums = [-1,2,3]
 * 。
 *
 *
 * 数组的 MEX (minimum excluded) 是指其中数组中缺失的最小非负整数。
 *
 *
 * 例如，[-1,2,3] 的 MEX 是 0 ，而 [1,0,3] 的 MEX 是 2 。
 *
 *
 * 返回在执行上述操作 任意次 后，nums 的最大 MEX 。
 *
 *
 *
 * 示例 1：
 *
 * 输入：nums = [1,-10,7,13,6,8], value = 5
 * 输出：4
 * 解释：执行下述操作可以得到这一结果：
 * - nums[1] 加上 value 两次，nums = [1,0,7,13,6,8]
 * - nums[2] 减去 value 一次，nums = [1,0,2,13,6,8]
 * - nums[3] 减去 value 两次，nums = [1,0,2,3,6,8]
 * nums 的 MEX 是 4 。可以证明 4 是可以取到的最大 MEX 。
 *
 *
 * 示例 2：
 *
 * 输入：nums = [1,-10,7,13,6,8], value = 7
 * 输出：2
 * 解释：执行下述操作可以得到这一结果：
 * - nums[2] 减去 value 一次，nums = [1,-10,0,13,6,8]
 * nums 的 MEX 是 2 。可以证明 2 是可以取到的最大 MEX 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length, value <= 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, mod_value: i32) -> i32 {
        // 从0开始...不断填, 直到找到缺失的那个正整数
        // 对value取模/取余, 得到相同的分组 --> 碰到同余的就消耗一个, 直到消耗到0, 即断开, 最大缺失的数
        // 由于键是从0开始的整数, 用数组更高效, 直接索引作为键, 操作idx: usize
        let mut cnt_map: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut cnt_map, &x| {
            // 这里需要额外加上, 是因为偏移处理负数 --> 负数加到最小正数
            // -2 + 5 = 3
            *cnt_map.entry((x % mod_value + mod_value) % mod_value).or_insert(0) += 1;
            cnt_map
        });

        // 从0开始逐个填坑, 同组消耗, 直到空缺
        // 不适合写成迭代器: iter::fron_fn + take_while + last
        for x in 0.. {
            if let Some(c) = cnt_map.get_mut(&(x % mod_value)) {
                if *c > 0 {
                    *c -= 1; // 获取可变引用, 消耗一个数字
                    continue;
                }
            }
            return x; // 说明缺少值, 或者统计的消耗完了, 已经能到达的最大值
        }
        unreachable!()
    }
}
// @lc code=end
