/*
 * @lc app=leetcode.cn id=1304 lang=rust
 *
 * [1304] 和为零的 N 个不同整数
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n: usize = n as usize; // 索引
        let mut arr: Vec<i32> = vec![0;n];
        let m: usize = n / 2; // 折半遍历, 奇数中间位为0
        for i in 0..m {
            arr[i] = i as i32 + 1; // 从1开始
            arr[i + m] = -arr[i]; // 对称赋值
        }
        arr
    }
}
// @lc code=end

