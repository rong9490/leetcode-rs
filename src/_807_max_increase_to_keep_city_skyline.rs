/*
 * @lc app=leetcode.cn id=807 lang=rust
 *
 * [807] 保持城市天际线
 *
 * https://leetcode.cn/problems/max-increase-to-keep-city-skyline/description/
 *
 * algorithms
 * Medium (88.78%)
 * Likes:    313
 * Dislikes: 0
 * Total Accepted:    73.8K
 * Total Submissions: 83.2K
 * Testcase Example:  '[[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]'
 *
 * 给你一座由 n x n 个街区组成的城市，每个街区都包含一座立方体建筑。给你一个下标从 0 开始的 n x n 整数矩阵 grid ，其中
 * grid[r][c] 表示坐落于 r 行 c 列的建筑物的 高度 。
 *
 * 城市的 天际线 是从远处观察城市时，所有建筑物形成的外部轮廓。从东、南、西、北四个主要方向观测到的 天际线 可能不同。
 *
 * 我们被允许为 任意数量的建筑物 的高度增加 任意增量（不同建筑物的增量可能不同） 。 高度为 0 的建筑物的高度也可以增加。然而，增加的建筑物高度
 * 不能影响 从任何主要方向观察城市得到的 天际线 。
 *
 * 在 不改变 从任何主要方向观测到的城市 天际线 的前提下，返回建筑物可以增加的 最大高度增量总和 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：grid = [[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]
 * 输出：35
 * 解释：建筑物的高度如上图中心所示。
 * 用红色绘制从不同方向观看得到的天际线。
 * 在不影响天际线的情况下，增加建筑物的高度：
 * gridNew = [ [8, 4, 8, 7],
 * ⁠           [7, 4, 7, 7],
 * ⁠           [9, 4, 8, 7],
 * ⁠           [3, 3, 3, 3] ]
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [[0,0,0],[0,0,0],[0,0,0]]
 * 输出：0
 * 解释：增加任何建筑物的高度都会导致天际线的变化。
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == grid.length
 * n == grid[r].length
 * 2 <= n <= 50
 * 0 <= grid[r][c] <= 100
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        // (二维数组)登记每一格: 从左往右看依次最大值 与 从上往下看依次最大值 "前缀最大值"
        let mut row_max: Vec<i32> = vec![0; grid.len()];
        let mut col_max: Vec<i32> = vec![0; grid[0].len()];

        // 遍历每行
        for (i, row) in grid.iter().enumerate() {
            // 遍历每格
            for (j, &x) in row.iter().enumerate() {
                // 求出每一格从两个方向看的最大值
                row_max[i] = row_max[i].max(x); // 当前值 -> 行上最大值比较
                col_max[j] = col_max[j].max(x); // 当前值 -> 列上最大值比较
            }
        }

        let mut ans: i32 = 0;
        // 遍历每行
        for (i, row) in grid.iter().enumerate() {
            // 遍历每格
            for (j, &x) in row.iter().enumerate() {
                // 两个方向的最大值的最小值, 减去当前的, 就是可增加的!
                // 两个方向, 只有添加其中小值, 才能不影响另一边, 否则调整了大的, 小的就被改变了!
                ans += row_max[i].min(col_max[j]) - x;
            }
        }
        ans
    }
}
// @lc code=end
