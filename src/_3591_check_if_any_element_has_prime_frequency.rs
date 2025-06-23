/*
 * @lc app=leetcode.cn id=3591 lang=rust
 *
 * [3591] 检查元素频次是否为质数
 *
 * https://leetcode.cn/problems/check-if-any-element-has-prime-frequency/description/
 *
 * algorithms
 * Easy (59.42%)
 * Likes:    1
 * Dislikes: 0
 * Total Accepted:    2.1K
 * Total Submissions: 3.4K
 * Testcase Example:  '[1,2,3,4,5,4]'
 *
 * 给你一个整数数组 nums。
 *
 * 如果数组中任一元素的 频次 是 质数，返回 true；否则，返回 false。
 *
 * 元素 x 的 频次 是它在数组中出现的次数。
 *
 * 质数是一个大于 1 的自然数，并且只有两个因数：1 和它本身。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入： nums = [1,2,3,4,5,4]
 *
 * 输出： true
 *
 * 解释：
 *
 * 数字 4 的频次是 2，而 2 是质数。
 *
 *
 * 示例 2：
 *
 *
 * 输入： nums = [1,2,3,4,5]
 *
 * 输出： false
 *
 * 解释：
 *
 * 所有元素的频次都是 1。
 *
 *
 * 示例 3：
 *
 *
 * 输入： nums = [2,2,2,4,4]
 *
 * 输出： true
 *
 * 解释：
 *
 * 数字 2 和 4 的频次都是质数。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start

use std::collections::HashMap;
// use std::sync::Once;

// 用埃氏筛（或者欧拉筛）预处理一个布尔数组，表示哪些数是质数。
const MX: i32 = 101; // 最大值
// static mut NP: [bool; MX] = [false; MX];
// static INIT: Once = Once::new();

// fn initialize() {
//     INIT.call_once(|| {
//         unsafe {
//             NP[1] = true;
//             for i in 2..MX {
//                 if i * i >= MX {
//                     break;
//                 }
//                 if !NP[i] {
//                     let mut j = i * i;
//                     while j < MX {
//                         NP[j] = true; // j 是质数 i 的倍数
//                         j += i;
//                     }
//                 }
//             }
//         }
//     });
// }

const PRIMES: [i32; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        // 计算频数
        let mut cnt: HashMap<i32, i32> = std::collections::HashMap::new();
        for &x in &nums {
            *cnt.entry(x).or_insert(0) += 1;
        }

        // 判断存在
        for (_, &c) in cnt.iter() {
            if c < MX && PRIMES.contains(&c) {
                return true;
            }
        }
        false
    }
}
// @lc code=end
