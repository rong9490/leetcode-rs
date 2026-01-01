/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_shadow: Vec<i32> = Vec::from(digits); // 副本
        let size: usize = digits_shadow.len();
        let idxs = (0..size).rev(); // 序号倒序

        for i in idxs {
            let val: i32 = digits_shadow[i];
            if val != 9 {
                digits_shadow[i] += 1; // 最高位+1, 无需进位
                return digits_shadow;
            }
            // 需要进位: 当前位置置0, 上面(下一位)自动+1
            digits_shadow[i] = 0;
        }

        // 如果到这里还需要进1, 说明前面的位数不够, 如 99999
        digits_shadow.insert(0, 1);
        digits_shadow
    }
}
// @lc code=end
