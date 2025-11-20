/*
 * @lc app=leetcode.cn id=1859 lang=rust
 *
 * [1859] 将句子排序
 *
 * https://leetcode.cn/problems/sorting-the-sentence/description/
 *
 * algorithms
 * Easy (74.06%)
 * Likes:    40
 * Dislikes: 0
 * Total Accepted:    21.4K
 * Total Submissions: 28.9K
 * Testcase Example:  '"is2 sentence4 This1 a3"'
 *
 * 一个 句子 指的是一个序列的单词用单个空格连接起来，且开头和结尾没有任何空格。每个单词都只包含小写或大写英文字母。
 *
 * 我们可以给一个句子添加 从 1 开始的单词位置索引 ，并且将句子中所有单词 打乱顺序 。
 *
 *
 * 比方说，句子 "This is a sentence" 可以被打乱顺序得到 "sentence4 a3 is2 This1" 或者 "is2
 * sentence4 This1 a3" 。
 *
 *
 * 给你一个 打乱顺序 的句子 s ，它包含的单词不超过 9 个，请你重新构造并得到原本顺序的句子。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "is2 sentence4 This1 a3"
 * 输出："This is a sentence"
 * 解释：将 s 中的单词按照初始位置排序，得到 "This1 is2 a3 sentence4" ，然后删除数字。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "Myself2 Me1 I4 and3"
 * 输出："Me Myself and I"
 * 解释：将 s 中的单词按照初始位置排序，得到 "Me1 Myself2 and3 I4" ，然后删除数字。
 *
 *
 *
 * 提示：
 *
 *
 * 2
 * s 只包含小写和大写英文字母、空格以及从 1 到 9 的数字。
 * s 中单词数目为 1 到 9 个。
 * s 中的单词由单个空格分隔。
 * s 不包含任何前导或者后缀空格。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        // let s_iter = s.split_whitespace(); // 整个字符串按空白切分, 转为迭代器
        // let size: usize = s_iter.clone().count(); // count消耗了迭代器, 需要克隆
        // let mut ans_vec: Vec<String> = vec!["".to_string(); size]; // 结果的字符串容器, 最终合并为完整句子
        // for raw_sentence in s_iter {
        //     /* 如何解析出末尾的那一个字符, 判断其序号 */
        //     // 最后一位字符
        //     let last_chr: char = raw_sentence.chars().last().unwrap();
        //     let last_str: String = last_chr.to_string();
        //     // 参数需要为&str
        //     let last_no: usize = usize::from_str_radix(&last_str, 10).unwrap();
        //     let sentence: String = raw_sentence.trim_end_matches(|c| '0' < c && c <= '9').to_string();
        //     ans_vec[last_no - 1] = sentence
        // }
        // ans_vec.join(" ")

        let words: Vec<&str> = s.split_whitespace().collect();
        let mut sorted: Vec<String> = vec![String::new(); words.len()];

        for word in words {
            let len = word.len();
            let last_char = word.chars().last().unwrap(); // 题目保证有数字，所以unwrap安全
            let order = last_char.to_digit(10).unwrap() as usize; // 1~9
            let content = &word[..len - 1];
            sorted[order - 1] = content.to_string();
        }

        sorted.join(" ")
    }
}
// @lc code=end
