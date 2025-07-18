/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 *
 * https://leetcode.cn/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (44.88%)
 * Likes:    3354
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 3.4M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 * 
 * 如果不存在公共前缀，返回空字符串 ""。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：strs = ["flower","flow","flight"]
 * 输出："fl"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：strs = ["dog","racecar","car"]
 * 输出：""
 * 解释：输入不存在公共前缀。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] 如果非空，则仅由小写英文字母组成
 * 
 * 
 */

struct Solution;

// @lc code=start
use std::str::Bytes;
use std::string::String;
use std::iter::Enumerate;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let s0: &String = &strs[0]; // 第一字符串为基准
        let chars: Bytes<'_> = s0.bytes(); // 转字符
        let chars_enumerate: Enumerate<Bytes<'_>> = chars.enumerate(); // 转迭代器
        // 遍历每个字符
        for (i, char) in chars_enumerate {
            // 从第二个开始遍历字符串
            for str in &strs[1..] {
                // 当前字符串较短, 已经结束了, 提前返回
                if i == str.len() {
                    return s0[0..i].to_string();
                }
                let str_chars: &[u8] = str.as_bytes();
                // 出现第一个不同字符, 直接结束
                if str_chars[i] != char {
                    return s0[0..i].to_string();
                }
            }
        }
        s0.clone()
    }
}
// @lc code=end

