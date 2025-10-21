/*
 * @lc app=leetcode.cn id=1002 lang=rust
 *
 * [1002] 查找共用字符
 *
 * https://leetcode.cn/problems/find-common-characters/description/
 *
 * algorithms
 * Easy (70.46%)
 * Likes:    397
 * Dislikes: 0
 * Total Accepted:    106.8K
 * Total Submissions: 151.6K
 * Testcase Example:  '["bella","label","roller"]'
 *
 * 给你一个字符串数组 words ，请你找出所有在 words 的每个字符串中都出现的共用字符（包括重复字符），并以数组形式返回。你可以按 任意顺序
 * 返回答案。
 *
 *
 * 示例 1：
 *
 *
 * 输入：words = ["bella","label","roller"]
 * 输出：["e","l","l"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：words = ["cool","lock","cook"]
 * 输出：["c","o"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= words.length <= 100
 * 1 <= words[i].length <= 100
 * words[i] 由小写英文字母组成
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut stats: [usize; 26] = [usize::MAX; 26]; // 数组表, 0-25, 对应a-z字母, 以a为基准映射
        for word in &words {
            // 当前单词的全部字母频数
            let stats_tmp: [usize; 26] = word.chars().fold([0; 26], |mut acc, c| {
                let idx = (c as usize) - ('a' as usize);
                acc[idx] += 1;
                acc
            });
            // 比较为最小的频数 --> 合并到最终stats上
            for (i, &cnt) in stats_tmp.iter().enumerate() {
                if cnt > 0 {
                    stats[i] = stats[i].min(cnt);
                }
            }
        }

        let ret: Vec<String> = stats
            .iter()
            .enumerate()
            .flat_map(|(i, &count)| {
                let chr: char = (i as u8 + b'a') as char;
                let chr_string: String = String::from(chr);
                // 字符重复count次
                std::iter::repeat(chr_string).take(count)
            })
            // 迭代器聚合回容器
            .collect::<Vec<String>>();
        ret
    }
}
// @lc code=end
