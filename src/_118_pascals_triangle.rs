/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 *
 * https://leetcode.cn/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (77.65%)
 * Likes:    1277
 * Dislikes: 0
 * Total Accepted:    701K
 * Total Submissions: 900.6K
 * Testcase Example:  '5'
 *
 * 给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。
 *
 * 在「杨辉三角」中，每个数是它左上方和右上方的数的和。
 *
 *
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: numRows = 5
 * 输出: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 *
 *
 * 示例 2:
 *
 *
 * 输入: numRows = 1
 * 输出: [[1]]
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n: usize = num_rows as usize;
        let mut c: Vec<Vec<i32>> = vec![vec![]; n]; // 二维数组

        // 二维循环
        for i in 0..n {
            // 调整容器的大小为i+1, 值为1
            c[i].resize(i + 1, 1);
            for j in 1..i {
                // 左上方的数 + 右上方的数
                c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
            }
        }
        c
    }
}
// @lc code=end
