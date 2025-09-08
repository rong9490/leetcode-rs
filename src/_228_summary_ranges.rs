/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 *
 * https://leetcode.cn/problems/summary-ranges/description/
 *
 * algorithms
 * Easy (54.92%)
 * Likes:    455
 * Dislikes: 0
 * Total Accepted:    230.7K
 * Total Submissions: 420.1K
 * Testcase Example:  '[0,1,2,4,5,7]'
 *
 * 给定一个  无重复元素 的 有序 整数数组 nums 。
 * 
 * 区间 [a,b] 是从 a 到 b（包含）的所有整数的集合。
 * 
 * 返回 恰好覆盖数组中所有数字 的 最小有序 区间范围列表 。也就是说，nums 的每个元素都恰好被某个区间范围所覆盖，并且不存在属于某个区间但不属于
 * nums 的数字 x 。
 * 
 * 列表中的每个区间范围 [a,b] 应该按如下格式输出：
 * 
 * 
 * "a->b" ，如果 a != b
 * "a" ，如果 a == b
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [0,1,2,4,5,7]
 * 输出：["0->2","4->5","7"]
 * 解释：区间范围是：
 * [0,2] --> "0->2"
 * [4,5] --> "4->5"
 * [7,7] --> "7"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [0,2,3,4,6,8,9]
 * 输出：["0","2->4","6","8->9"]
 * 解释：区间范围是：
 * [0,0] --> "0"
 * [2,4] --> "2->4"
 * [6,6] --> "6"
 * [8,9] --> "8->9"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= nums.length <= 20
 * -2^31 <= nums[i] <= 2^31 - 1
 * nums 中的所有值都 互不相同
 * nums 按升序排列
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let size: usize = nums.len();
        // 不重复/有序的 --> 单增;  '是否连续' --> 相差是否为1; 特殊的孤立断点
        let mut ans: Vec<String> = vec![];

        let mut i: usize = 0_usize; // 总体下标
        while i < size {
            let mut j: usize = i; // 尝试移动, 直到断开, 形成范围
            while j < size - 1 && nums[j] + 1 == nums[j + 1] {
                j += 1;
            }
            Self::add_range(&mut ans, nums[i], nums[j]);
            i = j + 1; // 新范围起点
        }
        ans
    }

    pub fn add_range(ans: &mut Vec<String>, val1: i32, val2: i32) {
        // 孤立的断点
        if val1 == val2 {
            ans.push(val1.to_string());
        } else {
            ans.push(format!("{}->{}", val1, val2));
        }
    }
}
// @lc code=end

