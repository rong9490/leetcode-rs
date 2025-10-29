/*
 * @lc app=leetcode.cn id=2315 lang=rust
 *
 * [2315] 统计星号
 *
 * https://leetcode.cn/problems/count-asterisks/description/
 *
 * algorithms
 * Easy (84.80%)
 * Likes:    67
 * Dislikes: 0
 * Total Accepted:    43.3K
 * Total Submissions: 51.1K
 * Testcase Example:  '"l|*e*et|c**o|*de|"'
 *
 * 给你一个字符串 s ，每 两个 连续竖线 '|' 为 一对 。换言之，第一个和第二个 '|' 为一对，第三个和第四个 '|' 为一对，以此类推。
 *
 * 请你返回 不在 竖线对之间，s 中 '*' 的数目。
 *
 * 注意，每个竖线 '|' 都会 恰好 属于一个对。
 *
 *
 *
 * 示例 1：
 *
 * 输入：s = "l|*e*et|c**o|*de|"
 * 输出：2
 * 解释：不在竖线对之间的字符加粗加斜体后，得到字符串："l|*e*et|c**o|*de|" 。
 * 第一和第二条竖线 '|' 之间的字符不计入答案。
 * 同时，第三条和第四条竖线 '|' 之间的字符也不计入答案。
 * 不在竖线对之间总共有 2 个星号，所以我们返回 2 。
 *
 * 示例 2：
 *
 * 输入：s = "iamprogrammer"
 * 输出：0
 * 解释：在这个例子中，s 中没有星号。所以返回 0 。
 *
 *
 * 示例 3：
 *
 * 输入：s = "yo|uar|e**|b|e***au|tifu|l"
 * 输出：5
 * 解释：需要考虑的字符加粗加斜体后："yo|uar|e**|b|e***au|tifu|l" 。不在竖线对之间总共有 5 个星号。所以我们返回 5
 * 。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 1000
 * s 只包含小写英文字母，竖线 '|' 和星号 '*' 。
 * s 包含 偶数 个竖线 '|' 。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        // s.as_bytes().iter().fold((false, 0), |(first, cnt), &ch| (if ch == b'|' { !first } else { first }, cnt + if !first && ch == b'*' { 1 } else { 0 })).1

        // 按字节依次处理
        let chr_bytes = s.as_bytes().iter();

        // 开始累计: 维护一个正反切换的flag, 表示是否有效的状态, 初始为true
        let pair: (bool, i32) = chr_bytes.fold((true, 0), |(flag, cnt), &chr| {
            let flag: bool = if chr == b'|' as u8 { !flag } else { flag };
            let cnt: i32 = if flag && chr == b'*' as u8 { cnt + 1 } else { cnt };
            (flag, cnt)
        });
        pair.1
    }
}
// @lc code=end
