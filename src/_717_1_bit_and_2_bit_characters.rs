/*
 * @lc app=leetcode.cn id=717 lang=rust
 *
 * [717] 1 比特与 2 比特字符
 *
 * https://leetcode.cn/problems/1-bit-and-2-bit-characters/description/
 *
 * algorithms
 * Easy (55.51%)
 * Likes:    312
 * Dislikes: 0
 * Total Accepted:    76.9K
 * Total Submissions: 138.6K
 * Testcase Example:  '[1,0,0]'
 *
 * 有两种特殊字符：
 *
 *
 * 第一种字符可以用一比特 0 表示
 * 第二种字符可以用两比特（10 或 11）表示
 *
 *
 * 给你一个以 0 结尾的二进制数组 bits ，如果最后一个字符必须是一个一比特字符，则返回 true 。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: bits = [1, 0, 0]
 * 输出: true
 * 解释: 唯一的解码方式是将其解析为一个两比特字符和一个一比特字符。
 * 所以最后一个字符是一比特字符。
 *
 *
 * 示例 2:
 *
 *
 * 输入：bits = [1,1,1,0]
 * 输出：false
 * 解释：唯一的解码方式是将其解析为两比特字符和两比特字符。
 * 所以最后一个字符不是一比特字符。
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= bits.length <= 1000
 * bits[i] 为 0 或 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        // 很难理解, 改为贪心
        // let mut flag: bool = true;
        // for &ch in bits.iter().rev().skip(1) {
        //     match ch {
        //         0 => return flag,
        //         1 => flag = !flag,
        //         _ => unreachable!(),
        //     }
        // }
        // flag

        // bits.iter()
        //     .rev() // 倒序
        //     .skip(1) // 跳过最后一个元素
        //     .fold(true, |flag, &ch| {
        //         // 维护 flag
        //         match ch {
        //             0 => flag,  // 遇到 0，返回当前 flag（不再提前返回，但逻辑等价）
        //             1 => !flag, // 遇到 1，翻转 flag
        //             _ => unreachable!(),
        //         }
        //     })
        // 依然反转, 统计1的个数,
        // let mut cnt: i32 = 0;
        // for i in (0..bits.len() - 1).rev() {
        //     if bits[i] == 1 {
        //         cnt += 1
        //     } else {
        //         break;
        //     }
        // }
        // cnt % 2 == 0

        let size: usize = bits.len();
        let mut i = 0;
        while i < size - 1 {
            // 循环直到剩下至多一个数字
            i += (bits[i] + 1) as usize; // 如果 bits[i] == 1 则跳过下一位
        }
        i == size - 1 // 注意题目保证 bits[n-1] == 0，无需判断

        // 理解, 一次性可以走一步, 或者走两步, 走到n-1的位置
        // pos = 0
        // while pos < len(bits) - 1:
        //   pos += 2 if bits[pos] == 1 else 1
        // return pos == len(bits) - 1
    }
}
// @lc code=end
