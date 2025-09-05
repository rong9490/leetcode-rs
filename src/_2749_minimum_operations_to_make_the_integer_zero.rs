/*
 * @lc app=leetcode.cn id=2749 lang=rust
 *
 * [2749] 得到整数零需要执行的最少操作数
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        // i32过不了这个用例, 需要拓展到i64 --> 1到60范围, 2^64位
        //  112577768, -501662198
        let mut k: i64 = 1; // 从1开始选...逐步自增
        loop {
            let x: i64 = num1 as i64 - num2 as i64 * k; // 计算差值: 3中情况

            // 小于零, 表示不够减, 返回-1
            if x < k {
                return -1;
            }

            // 剩余x的二进制中1的个数, 因为2^每次消除一位1
            let count_ones: i64 = x.count_ones() as i64;
            // 当k >= count_ones 说明k可以覆盖x减成0的最少次数
            if k >= count_ones {
                return k as i32;
            }
            k += 1;
        }
        unreachable!()
    }
}
// @lc code=end

