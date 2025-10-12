/*
 * @lc app=leetcode.cn id=2433 lang=rust
 *
 * [2433] 找出前缀异或的原始数组
 *
 * https://leetcode.cn/problems/find-the-original-array-of-prefix-xor/description/
 *
 * algorithms
 * Medium (86.00%)
 * Likes:    31
 * Dislikes: 0
 * Total Accepted:    13.4K
 * Total Submissions: 15.6K
 * Testcase Example:  '[5,2,0,3,1]'
 *
 * 给你一个长度为 n 的 整数 数组 pref 。找出并返回满足下述条件且长度为 n 的数组 arr ：
 *
 *
 * pref[i] = arr[0] ^ arr[1] ^ ... ^ arr[i].
 *
 *
 * 注意 ^ 表示 按位异或（bitwise-xor）运算。
 *
 * 可以证明答案是 唯一 的。
 *
 *
 *
 * 示例 1：
 *
 * 输入：pref = [5,2,0,3,1]
 * 输出：[5,7,2,3,2]
 * 解释：从数组 [5,7,2,3,2] 可以得到如下结果：
 * - pref[0] = 5
 * - pref[1] = 5 ^ 7 = 2
 * - pref[2] = 5 ^ 7 ^ 2 = 0
 * - pref[3] = 5 ^ 7 ^ 2 ^ 3 = 3
 * - pref[4] = 5 ^ 7 ^ 2 ^ 3 ^ 2 = 1
 *
 *
 * 示例 2：
 *
 * 输入：pref = [13]
 * 输出：[13]
 * 解释：pref[0] = arr[0] = 13
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= pref.length <= 10^5
 * 0 <= pref[i] <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        // 每一位都是从头异或到改位上...
        // 自反性 x ^ x = 0

        // let n: usize = pref.len();
        // let mut ret: Vec<i32> = vec![0; n];
        // ret[0] = pref[0];
        // for i in 1..n {
        //     // 为什么前面的不纳入计算, 因为会重复, 异或两次相互抵消了, 只剩
        //     ret[i] = pref[i - 1] ^ pref[i];
        // }
        // ret

        // 迭代器写法 vec -> iter -> vec
        pref.iter()
            .enumerate()
            .map(|(i, &x)| if i == 0 { x } else { pref[i - 1] ^ x })
            .collect()
    }
}
// @lc code=end
