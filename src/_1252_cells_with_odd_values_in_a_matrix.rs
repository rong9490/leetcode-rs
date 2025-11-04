/*
 * @lc app=leetcode.cn id=1252 lang=rust
 *
 * [1252] 奇数值单元格的数目
 *
 * https://leetcode.cn/problems/cells-with-odd-values-in-a-matrix/description/
 *
 * algorithms
 * Easy (79.75%)
 * Likes:    158
 * Dislikes: 0
 * Total Accepted:    56.2K
 * Total Submissions: 70.6K
 * Testcase Example:  '2\n3\n[[0,1],[1,1]]'
 *
 * 给你一个 m x n 的矩阵，最开始的时候，每个单元格中的值都是 0。
 *
 * 另有一个二维索引数组 indices，indices[i] = [ri, ci] 指向矩阵中的某个位置，其中 ri 和 ci 分别表示指定的行和列（从
 * 0 开始编号）。
 *
 * 对 indices[i] 所指向的每个位置，应同时执行下述增量操作：
 *
 *
 * ri 行上的所有单元格，加 1 。
 * ci 列上的所有单元格，加 1 。
 *
 *
 * 给你 m、n 和 indices 。请你在执行完所有 indices 指定的增量操作后，返回矩阵中 奇数值单元格 的数目。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：m = 2, n = 3, indices = [[0,1],[1,1]]
 * 输出：6
 * 解释：最开始的矩阵是 [[0,0,0],[0,0,0]]。
 * 第一次增量操作后得到 [[1,2,1],[0,1,0]]。
 * 最后的矩阵是 [[1,3,1],[1,3,1]]，里面有 6 个奇数。
 *
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入：m = 2, n = 2, indices = [[1,1],[0,0]]
 * 输出：0
 * 解释：最后的矩阵是 [[2,2],[2,2]]，里面没有奇数。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 1
 * 0 i < m
 * 0 i < n
 *
 *
 *
 *
 * 进阶：你可以设计一个时间复杂度为 O(n + m + indices.length) 且仅用 O(n + m) 额外空间的算法来解决此问题吗？
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        // 核心理解: 奇偶本质就是, true/false 的不断翻转
        let mut row: u64 = 0u64; // m,n最大50位 -> u64共64为, 可以容纳 --> 本质跟Vec::from_capcity(50)类似
        let mut col: u64 = 0u64;

        for e in &indices {
            let ri: u64 = e[0] as u64; // 行号
            let ci: u64 = e[1] as u64; // 列号
            row ^= 1u64 << ri; // 除了ri位不翻转, 其余翻转
            col ^= 1u64 << ci; // 除了ci列不翻转, 其余翻转
        }

        // 分别统计二进制中1的个数 --> 1代表的是true, 是偶数
        let cx: i32 = row.count_ones() as i32;
        let cy: i32 = col.count_ones() as i32;
        // true & false -> 奇数
        cx * (n - cy) + cy * (m - cx)
    }
}
// @lc code=end
