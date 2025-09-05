/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::from(digits); // 复制一份

        // 序号从到往小
        for i in (0..result.len()).rev() {
            let val: i32 = result[i];
            if val != 9 {
                result[i] += 1; // 最高位直接加了返回, 不会进位
                return result;
            } else {
                result[i] = 0; // 进位了, 当前位置为0 --> 最后再来补1
            }
        }
        // 走到这里: 一定是全是9, 不断进位到需要补位!
        result.insert(0, 1); // 在数组头补1
        result
    }
}
// @lc code=end
