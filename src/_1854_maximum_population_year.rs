/*
 * @lc app=leetcode.cn id=1854 lang=rust
 *
 * [1854] 人口最多的年份
 *
 * https://leetcode.cn/problems/maximum-population-year/description/
 *
 * algorithms
 * Easy (71.44%)
 * Likes:    91
 * Dislikes: 0
 * Total Accepted:    25.9K
 * Total Submissions: 36.2K
 * Testcase Example:  '[[1993,1999],[2000,2010]]'
 *
 * 给你一个二维整数数组 logs ，其中每个 logs[i] = [birthi, deathi] 表示第 i 个人的出生和死亡年份。
 * 
 * 年份 x 的 人口 定义为这一年期间活着的人的数目。第 i 个人被计入年份 x 的人口需要满足：x 在闭区间 [birthi, deathi - 1]
 * 内。注意，人不应当计入他们死亡当年的人口中。
 * 
 * 返回 人口最多 且 最早 的年份。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：logs = [[1993,1999],[2000,2010]]
 * 输出：1993
 * 解释：人口最多为 1 ，而 1993 是人口为 1 的最早年份。
 * 
 * 
 * 示例 2：
 * 
 * 输入：logs = [[1950,1961],[1960,1971],[1970,1981]]
 * 输出：1960
 * 解释： 
 * 人口最多为 2 ，分别出现在 1960 和 1970 。
 * 其中最早年份是 1960 。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= logs.length <= 100
 * 1950 <= birthi < deathi <= 2050
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        // 理解题目: 从每个人的开闭区间(左闭右开), 统计年份的人口数, 返回最早最大的年份(年份范围1950-2050)
        let start_year: i32 = 1950i32; // 开始年份
        let end_year: i32 = 2050i32; // 结束年份
        let wide_year: i32 = end_year - start_year + 1; // 区间大小
        // 映射成容器从0开始坐标
        let mut range_years: Vec<i32> = vec![0; wide_year as usize];

        // 登记每个人口范围
        for log in logs {
            let start: i32 = log[0];
            let end: i32 = log[1];
            let idx_start: i32 = start - start_year;
            let idx_end: i32 = end - start_year;
            // 应该是整个范围+1, 但是我们只需要取第一位, 所以仅下标开始+1, 下标结束-1
            range_years[idx_start as usize] += 1;
            range_years[idx_end as usize] -= 1;
        }

        // 开始统计最大及下标
        let mut cnt: i32 = 0; // 滚动统计
        let mut max_val: i32 = 0; // 最大处下标
        let mut max_idx: usize = 0; // 最大处的值
        for (idx, val) in range_years.into_iter().enumerate() {
            cnt += val;
            // 更新最大
            if cnt > max_val {
                max_val = cnt;
                max_idx = idx;
            }
        }

        // 还原目标年份加上起始年份1950
        start_year + (max_idx as i32)
    }
}
// @lc code=end