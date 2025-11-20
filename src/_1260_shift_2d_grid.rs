/*
 * @lc app=leetcode.cn id=1260 lang=rust
 *
 * [1260] 二维网格迁移
 *
 * https://leetcode.cn/problems/shift-2d-grid/description/
 *
 * algorithms
 * Easy (65.47%)
 * Likes:    136
 * Dislikes: 0
 * Total Accepted:    50.9K
 * Total Submissions: 77.8K
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]\n1'
 *
 * 给你一个 m 行 n 列的二维网格 grid 和一个整数 k。你需要将 grid 迁移 k 次。
 *
 * 每次「迁移」操作将会引发下述活动：
 *
 *
 * 位于 grid[i][j]（j < n - 1）的元素将会移动到 grid[i][j + 1]。
 * 位于 grid[i][n - 1] 的元素将会移动到 grid[i + 1][0]。
 * 位于 grid[m - 1][n - 1] 的元素将会移动到 grid[0][0]。
 *
 *
 * 请你返回 k 次迁移操作后最终得到的 二维网格。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
 * 输出：[[9,1,2],[3,4,5],[6,7,8]]
 *
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入：grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
 * 输出：[[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
 *
 *
 * 示例 3：
 *
 *
 * 输入：grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
 * 输出：[[1,2,3],[4,5,6],[7,8,9]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m <= 50
 * 1 <= n <= 50
 * -1000 <= grid[i][j] <= 1000
 * 0 <= k <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // 模拟整个流程: "迁移动作": 3个变化
        let (n, m) = (grid.len(), grid[0].len());
        let mut ret: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let k: usize = k as usize;
        let mut i: usize = k / m % n; // 取模简化重复的无效循环
        let mut j: usize = k % m;

        for row in grid.into_iter() {
            for v in row.into_iter() {
                ret[i][j] = v;
                j += 1;
                // 往右越界了, 回到0位置
                if j >= m {
                    j = 0;
                    i = (i + 1) % n;
                }
            }
        }
        ret
        // let (m, n) = (grid.len(), grid[0].len());
        // let total = m * n;
        // k %= total as i32;
        // let mut ret = vec![vec![0; n]; m];
        // let (mut i, mut j) = (0, total - k as usize);
        // while i < total {
        //     j = if j == total { 0 } else { j };
        //     ret[i / n][i % n] = grid[j / n][j % n];
        //     j += 1;
        //     i += 1;
        // }
        // ret
    }
}
// @lc code=end
