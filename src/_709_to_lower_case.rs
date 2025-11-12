/*
 * @lc app=leetcode.cn id=709 lang=rust
 *
 * [709] 转换成小写字母
 *
 * https://leetcode.cn/problems/to-lower-case/description/
 *
 * algorithms
 * Easy (76.42%)
 * Likes:    269
 * Dislikes: 0
 * Total Accepted:    200.4K
 * Total Submissions: 263.2K
 * Testcase Example:  '"Hello"'
 *
 * 给你一个字符串 s ，将该字符串中的大写字母转换成相同的小写字母，返回新的字符串。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "Hello"
 * 输出："hello"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "here"
 * 输出："here"
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "LOVELY"
 * 输出："lovely"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * s 由 ASCII 字符集中的可打印字符组成
 * 
 * 
 */

struct Solution;

// @lc code=start
use std::char::ToLowercase;
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let chars = s.chars(); // 按字符单个处理
        let ret: String = chars.flat_map(|chr| -> ToLowercase  {
            // if chr.is_ascii_uppercase() { chr } else { .. }
            chr.to_lowercase()
        }).collect::<String>();
        ret
    }
}
// @lc code=end

