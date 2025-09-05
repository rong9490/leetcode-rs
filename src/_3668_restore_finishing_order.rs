/*
 * @lc app=leetcode.cn id=3668 lang=rust
 *
 * [3668] Restore Finishing Order
 *
 * https://leetcode.cn/problems/restore-finishing-order/description/
 *
 * algorithms
 * Easy (90.43%)
 * Likes:    0
 * Dislikes: 0
 * Total Accepted:    2.3K
 * Total Submissions: 2.5K
 * Testcase Example:  '[3,1,2,5,4]\n[1,3,4]'
 *
 * You are given an integer array order of length n and an integer array
 * friends.
 * 
 * 
 * order contains every integer from 1 to n exactly once, representing the IDs
 * of the participants of a race in their finishing order.
 * friends contains the IDs of your friends in the race sorted in strictly
 * increasing order. Each ID in friends is guaranteed to appear in the order
 * array.
 * 
 * 
 * Return an array containing your friends' IDs in their finishing order.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: order = [3,1,2,5,4], friends = [1,3,4]
 * 
 * Output: [3,1,4]
 * 
 * Explanation:
 * 
 * The finishing order is [3, 1, 2, 5, 4]. Therefore, the finishing order of
 * your friends is [3, 1, 4].
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: order = [1,4,5,3,2], friends = [2,5]
 * 
 * Output: [5,2]
 * 
 * Explanation:
 * 
 * The finishing order is [1, 4, 5, 3, 2]. Therefore, the finishing order of
 * your friends is [5, 2].
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= n == order.length <= 100
 * order contains every integer from 1 to n exactly once
 * 1 <= friends.length <= min(8, n)
 * 1 <= friends[i] <= n
 * friends is strictly increasing
 * 
 * 
 */

struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        // into_iter = iter().cloned()
        let friends_set: HashSet<i32> = friends.into_iter().collect();
        // into_iter迭代器获取所有权
        order.into_iter()
            .filter(|id| friends_set.contains(id))
            .collect()
    }
}
// @lc code=end

