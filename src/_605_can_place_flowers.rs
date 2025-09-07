/*
 * @lc app=leetcode.cn id=605 lang=rust
 *
 * [605] 种花问题
 *
 * https://leetcode.cn/problems/can-place-flowers/description/
 *
 * algorithms
 * Easy (32.21%)
 * Likes:    779
 * Dislikes: 0
 * Total Accepted:    267.3K
 * Total Submissions: 830K
 * Testcase Example:  '[1,0,0,0,1]\n1'
 *
 * 假设有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花不能种植在相邻的地块上，它们会争夺水源，两者都会死去。
 *
 * 给你一个整数数组 flowerbed 表示花坛，由若干 0 和 1 组成，其中 0 表示没种植花，1 表示种植了花。另有一个数 n
 * ，能否在不打破种植规则的情况下种入 n 朵花？能则返回 true ，不能则返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：flowerbed = [1,0,0,0,1], n = 1
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：flowerbed = [1,0,0,0,1], n = 2
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= flowerbed.length <= 2 * 10^4
 * flowerbed[i] 为 0 或 1
 * flowerbed 中不存在相邻的两朵花
 * 0 <= n <= flowerbed.length
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let size: usize = flowerbed.len();
        // 贪心比较, 两端补0统一比较方式
        let mut arr: Vec<i32> = vec![0; size + 2]; // 多两位
        arr[1..(size + 1)].clone_from_slice(&flowerbed); // 从第一位开始, 覆盖进来

        let mut count: i32 = 0;
        // 从1开始, 安全的比较两侧
        for i in 1..(size + 1) {
            if arr[i - 1] == 0 && arr[i] == 0 && arr[i + 1] == 0 {
                arr[i] = 1; // 种上花
                count += 1;
            }
        }
        count >= n
    }
}
// @lc code=end
