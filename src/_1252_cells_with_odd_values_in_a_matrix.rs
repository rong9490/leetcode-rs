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
        // // 核心理解: 奇偶本质就是, true/false 的不断翻转
        // let mut row: u64 = 0u64; // m,n最大50位 -> u64共64为, 可以容纳 --> 本质跟Vec::from_capcity(50)类似
        // let mut col: u64 = 0u64;
        // for e in &indices {
        //     let ri: u64 = e[0] as u64; // 行号
        //     let ci: u64 = e[1] as u64; // 列号
        //     row ^= 1u64 << ri; // 除了ri位不翻转, 其余翻转
        //     col ^= 1u64 << ci; // 除了ci列不翻转, 其余翻转
        // }
        // // 分别统计二进制中1的个数 --> 1代表的是true, 是偶数
        // let cx: i32 = row.count_ones() as i32;
        // let cy: i32 = col.count_ones() as i32;
        // // true & false -> 奇数
        // cx * (n - cy) + cy * (m - cx)

        // // m行 / n列: 创建空白矩阵, 模拟翻转过程 (m作为行是在外层的)
        // let mut arr : Vec<Vec<i32>>= vec![vec![0; n as usize]; m as usize];
        // // 遍历"指令": [r, c], r代表行加1, c代表列加1
        // for cmd in indices {
        //     let r: usize = cmd[0] as usize;
        //     let c: usize = cmd[1] as usize;
        //     for i in 0..n {
        //         // 该行, 每一位格子加1
        //         arr[r as usize][i as usize] += 1;
        //     }
        //     for i in 0..m {
        //         // 该列, 每一个格子加1
        //         arr[i as usize][c as usize] += 1;
        //     }
        // }
        // let mut ans: i32 = 0i32;
        // for i in 0..m {
        //     for j in 0..n {
        //         if arr[i as usize][j as usize] % 2 == 1 { ans += 1 }
        //     }
        // }
        // ans

        let m: usize = m as usize;
        let n: usize = n as usize;
        let mut row_ops: Vec<i32> = vec![0i32; m];
        let mut col_ops: Vec<i32> = vec![0i32; n];
        for cmd in indices {
            let r: usize = cmd[0] as usize;
            let c: usize = cmd[1] as usize;
            row_ops[r] += 1;
            col_ops[c] += 1;
        }
        let odd_rows: usize = row_ops.iter().filter(|&&x| x % 2 == 1).count();
        let odd_cols: usize = col_ops.iter().filter(|&&x| x % 2 == 1).count();
        let even_rows: usize = m - odd_rows;
        let even_cols: usize = n - odd_cols;
        odd_rows as i32 * even_cols as i32 + even_rows as i32 * odd_cols as i32
    }
}
// @lc code=end
