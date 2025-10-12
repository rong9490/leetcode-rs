/*
 * @lc app=leetcode.cn id=1189 lang=rust
 *
 * [1189] “气球” 的最大数量
 *
 * https://leetcode.cn/problems/maximum-number-of-balloons/description/
 *
 * algorithms
 * Easy (68.20%)
 * Likes:    145
 * Dislikes: 0
 * Total Accepted:    67K
 * Total Submissions: 98.2K
 * Testcase Example:  '"nlaebolko"'
 *
 * 给你一个字符串 text，你需要使用 text 中的字母来拼凑尽可能多的单词 "balloon"（气球）。
 *
 * 字符串 text 中的每个字母最多只能被使用一次。请你返回最多可以拼凑出多少个单词 "balloon"。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：text = "nlaebolko"
 * 输出：1
 *
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入：text = "loonbalxballpoon"
 * 输出：2
 *
 *
 * 示例 3：
 *
 *
 * 输入：text = "leetcode"
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= text.length <= 10^4
 * text 全部由小写英文字母组成
 *
 *
 *
 *
 * 注意：本题与 2287. 重排字符形成目标字符串 相同。
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // 遍历text, 进行模式匹配 统计字符出现次数
        // let mut cnt: [i32; 5] = [0;5];
        // text.chars().for_each(|c| match c {
        //     'b' => cnt[0] += 1,
        //     'a' => cnt[1] += 1,
        //     'l' => cnt[2] += 1,
        //     'o' => cnt[3] += 1,
        //     'n' => cnt[4] += 1,
        //     _ => unreachable!(),
        // });
        // cnt[2] >>= 1; // 因为每次需要消耗两个字符
        // cnt[3] >>= 1; // 因为每次需要消耗两个字符
        // *cnt.iter().min().unwrap() // 取出现的最少次数, 作为单次的最大次数

        let counts: [i32; 5] = text
            .chars()
            // 匹配宏 matches!()  https://zhuanlan.zhihu.com/p/150699376
            .filter(|&c| matches!(c, 'b' | 'a' | 'l' | 'o' | 'n'))
            .fold([0; 5], |mut acc, c| {
                match c {
                    'b' => acc[0] += 1,
                    'a' => acc[1] += 1,
                    'l' => acc[2] += 1,
                    'o' => acc[3] += 1,
                    'n' => acc[4] += 1,
                    _ => unreachable!(),
                }
                acc
            });

        let max_balloons: i32 = *[
            counts[0],     // b
            counts[1],     // a
            counts[2] / 2, // l 消耗2个
            counts[3] / 2, // o 消耗2个
            counts[4],     // n
        ]
        .iter()
        .min()
        .unwrap();

        max_balloons
    }
}
// @lc code=end
