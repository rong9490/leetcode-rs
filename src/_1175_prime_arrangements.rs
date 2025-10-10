/*
 * @lc app=leetcode.cn id=1175 lang=rust
 *
 * [1175] 质数排列
 *
 * https://leetcode.cn/problems/prime-arrangements/description/
 *
 * algorithms
 * Easy (57.29%)
 * Likes:    129
 * Dislikes: 0
 * Total Accepted:    37K
 * Total Submissions: 64.6K
 * Testcase Example:  '5'
 *
 * 请你帮忙给从 1 到 n 的数设计排列方案，使得所有的「质数」都应该被放在「质数索引」（索引从 1 开始）上；你需要返回可能的方案总数。
 *
 * 让我们一起来回顾一下「质数」：质数一定是大于 1 的，并且不能用两个小于它的正整数的乘积来表示。
 *
 * 由于答案可能会很大，所以请你返回答案 模 mod 10^9 + 7 之后的结果即可。
 *
 *
 *
 * 示例 1：
 *
 * 输入：n = 5
 * 输出：12
 * 解释：举个例子，[1,2,5,4,3] 是一个有效的排列，但 [5,2,3,4,1] 不是，因为在第二种情况里质数 5 被错误地放在索引为 1
 * 的位置上。
 *
 *
 * 示例 2：
 *
 * 输入：n = 100
 * 输出：682289015
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        const MOD: i64 = 1000000007i64;
        // 分别统计质数与合数的个数
        let (mut ret, prime, not_prime) = (
            1,
            (3..=n).filter(|i| Solution::is_prime_fn(*i)).count() as i32 + 1,
            (3..=n).filter(|i| !Solution::is_prime_fn(*i)).count() as i32 + 1,
        );

        // 求阶乘, 每次都取模mod, 防止溢出
        ret = (1..=prime).fold(ret, |acc, i| (acc * i as i64) % MOD);
        ret = (1..=not_prime).fold(ret, |acc, i| (acc * i as i64) % MOD);
        ret as i32
    }

    // 迭代器all判断质数
    fn is_prime_fn(x: i32) -> bool {
        if x < 2 {
            return false;
        }
        if x == 2 {
            return true;
        }
        let sqrt_x: i32 = (x as f64).sqrt() as i32;
        // step_by(2)
        (2..=sqrt_x).all(|i: i32| x % i != 0)
    }
}
// @lc code=end
