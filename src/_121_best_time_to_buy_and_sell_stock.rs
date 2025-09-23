/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 *
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/description/
 *
 * algorithms
 * Easy (59.12%)
 * Likes:    3894
 * Dislikes: 0
 * Total Accepted:    1.9M
 * Total Submissions: 3.2M
 * Testcase Example:  '[7,1,5,3,6,4]'
 *
 * 给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。
 *
 * 你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。
 *
 * 返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：[7,1,5,3,6,4]
 * 输出：5
 * 解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
 * ⁠    注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
 *
 *
 * 示例 2：
 *
 *
 * 输入：prices = [7,6,4,3,1]
 * 输出：0
 * 解释：在这种情况下, 没有交易完成, 所以最大利润为 0。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 0
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut ans: i32 = 0;
        // let mut min_price: i32 = prices[0];
        // for p in prices {
        //     ans = ans.max(p - min_price); // 更新最大的差值
        //     min_price = min_price.min(p); // 更新最小的买入时机
        // }
        // ans

        if prices.is_empty() {
            return 0;
        }
        let first_price: i32 = prices[0]; // 先取出第一个价格，再 move vec 到迭代器
        let folded = prices
            .into_iter()
            .fold((first_price, 0), |(min_price, max_profit), p| {
                let new_min: i32 = min_price.min(p);
                let new_profit: i32 = (p - new_min).max(max_profit);
                (new_min, new_profit)
            });

        folded.1
    }
}
// @lc code=end
