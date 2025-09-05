/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // 用flod递推迭代 --> 维护一个数组的值不断累计 --> 是空间优化(原地)的动态规划
        (0..n).fold([1, 1], |way, _| [way[1], way[0] + way[1]])[0]
    }
}
// @lc code=end
