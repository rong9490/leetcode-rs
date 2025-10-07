/*
 * @lc app=leetcode.cn id=405 lang=rust
 *
 * [405] 数字转换为十六进制数
 *
 * https://leetcode.cn/problems/convert-a-number-to-hexadecimal/description/
 *
 * algorithms
 * Easy (54.97%)
 * Likes:    320
 * Dislikes: 0
 * Total Accepted:    80.7K
 * Total Submissions: 146.8K
 * Testcase Example:  '26'
 *
 * 给定一个整数，编写一个算法将这个数转换为十六进制数。对于负整数，我们通常使用 补码运算 方法。
 * 
 * 答案字符串中的所有字母都应该是小写字符，并且除了 0 本身之外，答案中不应该有任何前置零。
 * 
 * 注意: 不允许使用任何由库提供的将数字直接转换或格式化为十六进制的方法来解决这个问题。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：num = 26
 * 输出："1a"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：num = -1
 * 输出："ffffffff"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * -2^31 <= num <= 2^31 - 1
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn to_hex(num: i32) -> String {
        // 调用内置库: format!("{:x}", num)
        // TODO 没看懂, 迭代器 + char::from_digit()
        if num == 0 {
            return "0".to_string();
        }

        let mut ret: Vec<char> = Vec::new();
        for i in (0..8).map(|x| x * 4).rev() {
            let v: i32 = (num >> i) & 0xf;
            if ret.len() > 0 || v > 0 {
                let chr: char = char::from_digit(v as u32, 16).unwrap();
                ret.push(chr);
            }
        }

        ret.into_iter().collect::<String>()
    }
}
// @lc code=end

