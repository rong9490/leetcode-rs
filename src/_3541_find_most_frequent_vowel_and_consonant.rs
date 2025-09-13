/*
 * @lc app=leetcode.cn id=3541 lang=rust
 *
 * [3541] 找到频率最高的元音和辅音
 *
 * https://leetcode.cn/problems/find-most-frequent-vowel-and-consonant/description/
 *
 * algorithms
 * Easy (86.71%)
 * Likes:    11
 * Dislikes: 0
 * Total Accepted:    8.9K
 * Total Submissions: 9.8K
 * Testcase Example:  '"successes"'
 *
 * 给你一个由小写英文字母（'a' 到 'z'）组成的字符串 s。你的任务是找出出现频率 最高 的元音（'a'、'e'、'i'、'o'、'u'
 * 中的一个）和出现频率最高的辅音（除元音以外的所有字母），并返回这两个频率之和。
 *
 * 注意：如果有多个元音或辅音具有相同的最高频率，可以任选其中一个。如果字符串中没有元音或没有辅音，则其频率视为 0。
 * 一个字母 x 的 频率 是它在字符串中出现的次数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入: s = "successes"
 *
 * 输出: 6
 *
 * 解释:
 *
 *
 * 元音有：'u' 出现 1 次，'e' 出现 2 次。最大元音频率 = 2。
 * 辅音有：'s' 出现 4 次，'c' 出现 2 次。最大辅音频率 = 4。
 * 输出为 2 + 4 = 6。
 *
 *
 *
 * 示例 2：
 *
 *
 * 输入: s = "aeiaeia"
 *
 * 输出: 3
 *
 * 解释:
 *
 *
 * 元音有：'a' 出现 3 次，'e' 出现 2 次，'i' 出现 2 次。最大元音频率 = 3。
 * s 中没有辅音。因此，最大辅音频率 = 0。
 * 输出为 3 + 0 = 3。
 *
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= s.length <= 100
 * s 只包含小写英文字母
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        // 特殊情况, 可以用数组优化map
        // 统计两个最大频数
        // 遍历统计频数 (forin或迭代器)
        let vowels: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        let (cnt, max_vowel_cnt, max_consonant_cnt) = s.bytes().fold(
            ([0i32; 26], 0i32, 0i32),
            |(mut cnt, mut max_v, mut max_c), ch| {
                let idx = (ch - b'a') as usize;
                cnt[idx] += 1;

                if vowels.contains(&ch) {
                    max_v = max_v.max(cnt[idx]);
                } else {
                    max_c = max_c.max(cnt[idx]);
                }

                (cnt, max_v, max_c)
            },
        );
        max_vowel_cnt + max_consonant_cnt
    }
}
// @lc code=end
