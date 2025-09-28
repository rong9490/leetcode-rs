/*
 * @lc app=leetcode.cn id=976 lang=rust
 *
 * [976] 三角形的最大周长
 *
 * https://leetcode.cn/problems/largest-perimeter-triangle/description/
 *
 * algorithms
 * Easy (58.18%)
 * Likes:    310
 * Dislikes: 0
 * Total Accepted:    123.7K
 * Total Submissions: 210.1K
 * Testcase Example:  '[2,1,2]'
 *
 * 给定由一些正数（代表长度）组成的数组 nums ，返回 由其中三个长度组成的、面积不为零的三角形的最大周长 。如果不能形成任何面积不为零的三角形，返回
 * 0。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,1,2]
 * 输出：5
 * 解释：你可以用三个边长组成一个三角形:1 2 2。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,1,10]
 * 输出：0
 * 解释：
 * 你不能用边长 1,1,2 来组成三角形。
 * 不能用边长 1,1,10 来构成三角形。
 * 不能用边长 1、2 和 10 来构成三角形。
 * 因为我们不能用任何三条边长来构成一个非零面积的三角形，所以我们返回 0。
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= nums.length <= 10^4
 * 1 <= nums[i] <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable(); // 排序
                              // 求最大两边, 降序检验
                              // for i in (2..nums.len()).rev() {
                              //     // 三角形边长要求: 两边之和 > 第三边
                              //     if nums[i - 2] + nums[i - 1] > nums[i] {
                              //         return nums[i - 2] + nums[i - 1] + nums[i];
                              //     }
                              // }
                              // 0

        // 优化成迭代器: windows
        nums.windows(3) // 每次取连续的3个元素
            .rev() // 从后往前看，优先找大的组合
            .find(|w| {
                // w 是一个长度为3的切片 &[a, b, c]，且 a <= b <= c
                let a: i32 = w[0];
                let b: i32 = w[1];
                let c: i32 = w[2];
                a + b > c // 判断是否能构成三角形
            })
            .map(|w| w.iter().sum()) // 如果找到了，计算三边之和
            .unwrap_or(0) // 找不到就返回 0
    }
}
// @lc code=end
