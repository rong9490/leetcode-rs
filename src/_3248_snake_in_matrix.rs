/*
 * @lc app=leetcode.cn id=3248 lang=rust
 *
 * [3248] 矩阵中的蛇
 *
 * https://leetcode.cn/problems/snake-in-matrix/description/
 *
 * algorithms
 * Easy (88.75%)
 * Likes:    32
 * Dislikes: 0
 * Total Accepted:    25.2K
 * Total Submissions: 28.4K
 * Testcase Example:  '2\n["RIGHT","DOWN"]'
 *
 * 大小为 n x n 的矩阵 grid 中有一条蛇。蛇可以朝 四个可能的方向 移动。矩阵中的每个单元格都使用位置进行标识： grid[i][j] = (i
 * * n) + j。
 *
 * 蛇从单元格 0 开始，并遵循一系列命令移动。
 *
 * 给你一个整数 n 表示 grid 的大小，另给你一个字符串数组 commands，其中包括 "UP"、"RIGHT"、"DOWN" 和
 * "LEFT"。题目测评数据保证蛇在整个移动过程中将始终位于 grid 边界内。
 *
 * 返回执行 commands 后蛇所停留的最终单元格的位置。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 2, commands = ["RIGHT","DOWN"]
 *
 * 输出：3
 *
 * 解释：
 *
 *
 *
 *
 *
 * 0
 * 1
 *
 *
 * 2
 * 3
 *
 *
 *
 *
 *
 *
 *
 * 0
 * 1
 *
 *
 * 2
 * 3
 *
 *
 *
 *
 *
 *
 *
 * 0
 * 1
 *
 *
 * 2
 * 3
 *
 *
 *
 *
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 3, commands = ["DOWN","RIGHT","UP"]
 *
 * 输出：1
 *
 * 解释：
 *
 *
 *
 *
 *
 * 0
 * 1
 * 2
 *
 *
 * 3
 * 4
 * 5
 *
 *
 * 6
 * 7
 * 8
 *
 *
 *
 *
 *
 *
 *
 * 0
 * 1
 * 2
 *
 *
 * 3
 * 4
 * 5
 *
 *
 * 6
 * 7
 * 8
 *
 *
 *
 *
 *
 *
 *
 * 0
 * 1
 * 2
 *
 *
 * 3
 * 4
 * 5
 *
 *
 * 6
 * 7
 * 8
 *
 *
 *
 *
 *
 *
 *
 * 0
 * 1
 * 2
 *
 *
 * 3
 * 4
 * 5
 *
 *
 * 6
 * 7
 * 8
 *
 *
 *
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= n <= 10
 * 1 <= commands.length <= 100
 * commands 仅由 "UP"、"RIGHT"、"DOWN" 和 "LEFT" 组成。
 * 生成的测评数据确保蛇不会移动到矩阵的边界外。
 *
 *
 */

pub struct Solution;

// @lc code=start

fn direct_move(cmd: &str, i: i32, j: i32) -> (i32, i32) {
    // 只需要头一个字符判断
    match cmd.as_bytes()[0] {
        b'U' => (i - 1, j), // 上移
        b'D' => (i + 1, j), // 下移
        b'L' => (i, j - 1), // 左移
        b'R' => (i, j + 1), // 右移
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        // 模拟题: 遍历方向进行移动
        // 迭代器遍历
        let (i, j) = commands
            .iter()
            .fold((0, 0), |(i, j), cmd| direct_move(cmd, i, j));
        // 最终位置公式
        i * n + j
    }
}
// @lc code=end
