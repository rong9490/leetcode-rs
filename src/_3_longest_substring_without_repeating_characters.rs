/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (41.54%)
 * Likes:    11040
 * Dislikes: 0
 * Total Accepted:    3.7M
 * Total Submissions: 9M
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串 s ，请你找出其中不含有重复字符的 最长 子串 的长度。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: s = "abcabcbb"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 *
 *
 * 示例 2:
 *
 *
 * 输入: s = "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 *
 *
 * 示例 3:
 *
 *
 * 输入: s = "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= s.length <= 5 * 10^4
 * s 由英文字母、数字、符号和空格组成
 *
 *
 */

struct Solution;

// @lc code=start
use std::cmp;
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // TODO 优化理解
        let n: usize = s.len();
        let bytes: &[u8] = s.as_bytes();
        let mut length: usize = 0;
        // 记录滑动窗口内出现过的字符, 双指针
        let mut seen: HashSet<u8> = HashSet::new();
        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < n {
            let b: u8 = bytes[right];
            // 右指针所指的字符与滑动窗口内某个字符重复
            if seen.contains(&b) {
                length = cmp::max(length, right - left);
                // 左指针直接移动到滑动窗口内与右指针字符重复的字符的下一个位置
                while seen.contains(&b) {
                    seen.remove(&bytes[left]);
                    left += 1;
                }
            }
            seen.insert(b);
            right += 1;
        }

        length = cmp::max(length, right - left);
        length as i32
    }
}
// @lc code=end
