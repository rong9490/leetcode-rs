/*
 * @lc app=leetcode.cn id=374 lang=rust
 *
 * [374] 猜数字大小
 *
 * https://leetcode.cn/problems/guess-number-higher-or-lower/description/
 *
 * algorithms
 * Easy (52.97%)
 * Likes:    367
 * Dislikes: 0
 * Total Accepted:    215.9K
 * Total Submissions: 407.4K
 * Testcase Example:  '10\n6'
 *
 * 我们正在玩猜数字游戏。猜数字游戏的规则如下：
 * 
 * 我会从 1 到 n 随机选择一个数字。 请你猜选出的是哪个数字。（我选的数字在整个游戏中保持不变）。
 * 
 * 如果你猜错了，我会告诉你，我选出的数字比你猜测的数字大了还是小了。
 * 
 * 你可以通过调用一个预先定义好的接口 int guess(int num) 来获取猜测结果，返回值一共有三种可能的情况：
 * 
 * 
 * -1：你猜的数字比我选出的数字大 （即 num > pick）。
 * 1：你猜的数字比我选出的数字小 （即 num < pick）。
 * 0：你猜的数字与我选出的数字相等。（即 num == pick）。
 * 
 * 
 * 返回我选出的数字。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 10, pick = 6
 * 输出：6
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1, pick = 1
 * 输出：1
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：n = 2, pick = 1
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 2^31 - 1
 * 1 <= pick <= n
 * 
 * 
 */

struct Solution;

// @lc code=start
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        // 猜大小 --> 典型的二分查找场景, 需要维护两端的值(元组), 不断收缩
        // 技巧: 元组方便模式匹配
        let (mut l, mut r): (i32, i32) = (0, n);

        while l <= r {
            let mid: i32 = l + (r - l) / 2; // 计算中间值
            match guess(mid) {
                0 => return mid, // 相等
                -1 => r = mid - 1, // 偏小, 往右移动
                1 => l = mid + 1, // 偏大, 往左移动
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}
// @lc code=end

