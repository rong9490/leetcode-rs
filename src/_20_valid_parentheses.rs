/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 *
 * https://leetcode.cn/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (44.90%)
 * Likes:    4739
 * Dislikes: 0
 * Total Accepted:    2.2M
 * Total Submissions: 4.9M
 * Testcase Example:  '"()"'
 *
 * 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
 * 
 * 有效字符串需满足：
 * 
 * 
 * 左括号必须用相同类型的右括号闭合。
 * 左括号必须以正确的顺序闭合。
 * 每个右括号都有一个对应的相同类型的左括号。
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "()"
 * 
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "()[]{}"
 * 
 * 输出：true
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "(]"
 * 
 * 输出：false
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：s = "([])"
 * 
 * 输出：true
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 10^4
 * s 仅由括号 '()[]{}' 组成
 * 
 * 
 */

struct Solution;

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false; // 长度须为偶数
        }

        let map: HashMap<u8, u8> = [(b')', b'('), (b']', b'['), (b'}', b'{')].iter().cloned().collect::<HashMap<u8, u8>>();        
        let mut stack: Vec<u8> = vec![];

        for c in s.bytes() {
            if !map.contains_key(&c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    return false
                }
                let curr_right_char = map.get(&c).unwrap();
                let curr_right_char = *curr_right_char;
                let pop_char = stack.pop().unwrap();
                if curr_right_char != pop_char {
                    return false
                }
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

