/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] 验证回文串
 *
 * https://leetcode.cn/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (48.50%)
 * Likes:    839
 * Dislikes: 0
 * Total Accepted:    763.8K
 * Total Submissions: 1.6M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * 如果在将所有大写字符转换为小写字符、并移除所有非字母数字字符之后，短语正着读和反着读都一样。则可以认为该短语是一个 回文串 。
 * 
 * 字母和数字都属于字母数字字符。
 * 
 * 给你一个字符串 s，如果它是 回文串 ，返回 true ；否则，返回 false 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入: s = "A man, a plan, a canal: Panama"
 * 输出：true
 * 解释："amanaplanacanalpanama" 是回文串。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "race a car"
 * 输出：false
 * 解释："raceacar" 不是回文串。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = " "
 * 输出：true
 * 解释：在移除非字母数字字符之后，s 是一个空字符串 "" 。
 * 由于空字符串正着反着读都一样，所以是回文串。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 2 * 10^5
 * s 仅由可打印的 ASCII 字符组成
 * 
 * 
 */


struct Solution;

// @lc code=start
use regex::Regex;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
        let cleaned = re.replace_all(&s, ""); // 正则清理非字母数字
        let cleaned = cleaned.replace(r"\s+", "").to_lowercase(); // 清楚空白, 统一为小写
        let reversed: String = cleaned.chars().rev().collect::<String>(); // 翻转字符串

        cleaned == reversed // 比对是否回文
    }
}
// @lc code=end

