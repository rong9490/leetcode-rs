/*
 * @lc app=leetcode.cn id=89 lang=rust
 *
 * [89] 格雷编码
 *
 * https://leetcode.cn/problems/gray-code/description/
 *
 * algorithms
 * Medium (75.33%)
 * Likes:    710
 * Dislikes: 0
 * Total Accepted:    143.8K
 * Total Submissions: 190.9K
 * Testcase Example:  '2'
 *
 * n 位格雷码序列 是一个由 2^n 个整数组成的序列，其中：
 *
 * 每个整数都在范围 [0, 2^n - 1] 内（含 0 和 2^n - 1）
 * 第一个整数是 0
 * 一个整数在序列中出现 不超过一次
 * 每对 相邻 整数的二进制表示 恰好一位不同 ，且
 * 第一个 和 最后一个 整数的二进制表示 恰好一位不同
 *
 *
 * 给你一个整数 n ，返回任一有效的 n 位格雷码序列 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 2
 * 输出：[0,1,3,2]
 * 解释：
 * [0,1,3,2] 的二进制表示是 [00,01,11,10] 。
 * - 00 和 01 有一位不同
 * - 01 和 11 有一位不同
 * - 11 和 10 有一位不同
 * - 10 和 00 有一位不同
 * [0,2,3,1] 也是一个有效的格雷码序列，其二进制表示是 [00,10,11,01] 。
 * - 00 和 10 有一位不同
 * - 10 和 11 有一位不同
 * - 11 和 01 有一位不同
 * - 01 和 00 有一位不同
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：[0,1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 16
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // (0..1 << n).map(|i| (i >> 1) ^ i).collect()
        // (0..2_i32.pow(n as u32)).map(|x| x ^ x >> 1).collect()
        let mut n: i32 = n;
        let mut ret: Vec<i32> = vec![0];
        // 将n-1的结果镜像复制, 并在n-1的结果集数字二进制最后加上0, 镜像集数字二进制最后加上1, 得到n
        while n > 0 {
            n -= 1;
            let l: usize = ret.len();
            for i in (0..l).rev() {
                ret[i] <<= 1; // ret[i] *= 2
                ret.push(ret[i] ^ 1); // ret[i] + 1
            }
        }
        ret
    }
}
// @lc code=end
