/*
 * @lc app=leetcode.cn id=1832 lang=rust
 *
 * [1832] 判断句子是否为全字母句
 *
 * https://leetcode.cn/problems/check-if-the-sentence-is-pangram/description/
 *
 * algorithms
 * Easy (84.21%)
 * Likes:    89
 * Dislikes: 0
 * Total Accepted:    59.9K
 * Total Submissions: 71.2K
 * Testcase Example:  '"thequickbrownfoxjumpsoverthelazydog"'
 *
 * 全字母句 指包含英语字母表中每个字母至少一次的句子。
 *
 * 给你一个仅由小写英文字母组成的字符串 sentence ，请你判断 sentence 是否为 全字母句 。
 *
 * 如果是，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：sentence = "thequickbrownfoxjumpsoverthelazydog"
 * 输出：true
 * 解释：sentence 包含英语字母表中每个字母至少一次。
 *
 *
 * 示例 2：
 *
 *
 * 输入：sentence = "leetcode"
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * sentence 由小写英语字母组成
 *
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // 全26个字母, 因为按顺序, 可以映射为容器
        // let mut alphabet: Vec<bool> = vec![false; 26];
        // for ch in sentence.as_bytes() {
        //     let idx: usize = (ch - b'a') as usize;
        //     alphabet[idx] = true;
        // }
        // for i in 0..26 {
        //     if !alphabet[i] {
        //         return false;
        //     }
        // }
        // true

        let unique_letters: HashSet<char> = sentence
            .chars() // 转为字符遍历
            .filter(|ch| ch.is_ascii_lowercase()) // 只保留小写字母
            .collect::<HashSet<char>>(); // 转为HashSet
        unique_letters.len() == 26
    }
}
// @lc code=end
