/*
 * @lc app=leetcode.cn id=2798 lang=rust
 *
 * [2798] 满足目标工作时长的员工数目
 *
 * https://leetcode.cn/problems/number-of-employees-who-met-the-target/description/
 *
 * algorithms
 * Easy (88.82%)
 * Likes:    36
 * Dislikes: 0
 * Total Accepted:    34.8K
 * Total Submissions: 39.2K
 * Testcase Example:  '[0,1,2,3,4]\n2'
 *
 * 公司里共有 n 名员工，按从 0 到 n - 1 编号。每个员工 i 已经在公司工作了 hours[i] 小时。
 *
 * 公司要求每位员工工作 至少 target 小时。
 *
 * 给你一个下标从 0 开始、长度为 n 的非负整数数组 hours 和一个非负整数 target 。
 *
 * 请你用整数表示并返回工作至少 target 小时的员工数。
 *
 *
 *
 * 示例 1：
 *
 * 输入：hours = [0,1,2,3,4], target = 2
 * 输出：3
 * 解释：公司要求每位员工工作至少 2 小时。
 * - 员工 0 工作 0 小时，不满足要求。
 * - 员工 1 工作 1 小时，不满足要求。
 * - 员工 2 工作 2 小时，满足要求。
 * - 员工 3 工作 3 小时，满足要求。
 * - 员工 4 工作 4 小时，满足要求。
 * 共有 3 位满足要求的员工。
 *
 *
 * 示例 2：
 *
 * 输入：hours = [5,1,4,2,2], target = 6
 * 输出：0
 * 解释：公司要求每位员工工作至少 6 小时。
 * 共有 0 位满足要求的员工。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n == hours.length <= 50
 * 0 <= hours[i], target <= 10^5
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        // 迭代器过滤
        let h_iter = hours.iter().filter(|&&h| h >= target);
        // 只需要数量
        h_iter.count() as i32
    }
}
// @lc code=end
