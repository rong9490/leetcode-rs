/*
 * @lc app=leetcode.cn id=1812 lang=rust
 *
 * [1812] 判断国际象棋棋盘中一个格子的颜色
 *
 * https://leetcode.cn/problems/determine-color-of-a-chessboard-square/description/
 *
 * algorithms
 * Easy (82.81%)
 * Likes:    73
 * Dislikes: 0
 * Total Accepted:    65.7K
 * Total Submissions: 79.4K
 * Testcase Example:  '"a1"'
 *
 * 给你一个坐标 coordinates ，它是一个字符串，表示国际象棋棋盘中一个格子的坐标。下图是国际象棋棋盘示意图。
 *
 *
 *
 * 如果所给格子的颜色是白色，请你返回 true，如果是黑色，请返回 false 。
 *
 * 给定坐标一定代表国际象棋棋盘上一个存在的格子。坐标第一个字符是字母，第二个字符是数字。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：coordinates = "a1"
 * 输出：false
 * 解释：如上图棋盘所示，"a1" 坐标的格子是黑色的，所以返回 false 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：coordinates = "h3"
 * 输出：true
 * 解释：如上图棋盘所示，"h3" 坐标的格子是白色的，所以返回 true 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：coordinates = "c7"
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * coordinates.length == 2
 * 'a'
 * '1'
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        // 观察棋盘: 
        // 第一个点: a1为黑色(false), 把a转为0, 和为1为奇数
        // 向外延伸, 凡是和为奇数都是黑色, 偶数都是白色
        let str_bytes: &[u8] = coordinates.as_bytes();
        let column: i32 = (str_bytes[0] - b'a') as i32; // 从a起始
        let row: i32 = (str_bytes[1] - b'1') as i32; // 从1起始不是0
        (column + row) % 2 == 1 // 奇偶性决定颜色，奇数为白色
    }
}
// @lc code=end
