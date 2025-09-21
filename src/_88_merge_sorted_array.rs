/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 *
 * https://leetcode.cn/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (54.94%)
 * Likes:    2837
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 3.1M
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * 给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
 * 
 * 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
 * 
 * 注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m
 * 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 * 输出：[1,2,2,3,5,6]
 * 解释：需要合并 [1,2,3] 和 [2,5,6] 。
 * 合并结果是 [1,2,2,3,5,6] ，其中斜体加粗标注的为 nums1 中的元素。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums1 = [1], m = 1, nums2 = [], n = 0
 * 输出：[1]
 * 解释：需要合并 [1] 和 [] 。
 * 合并结果是 [1] 。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums1 = [0], m = 0, nums2 = [1], n = 1
 * 输出：[1]
 * 解释：需要合并的数组是 [] 和 [1] 。
 * 合并结果是 [1] 。
 * 注意，因为 m = 0 ，所以 nums1 中没有元素。nums1 中仅存的 0 仅仅是为了确保合并结果可以顺利存放到 nums1 中。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * nums1.length == m + n
 * nums2.length == n
 * 0 <= m, n <= 200
 * 1 <= m + n <= 200
 * -10^9 <= nums1[i], nums2[j] <= 10^9
 * 
 * 
 * 
 * 
 * 进阶：你可以设计实现一个时间复杂度为 O(m + n) 的算法解决此问题吗？
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 因为O(m + n), 只能遍历一次
        // 倒序避免数据覆盖, 将nums2值一个个比较与填入nums1
        let m: usize = m as usize;
        let n: usize = n as usize;
        let mut p1: usize = m - 1; // 倒序指针
        let mut p2: usize = n - 1;
        let mut p: usize = m + n - 1; // 新数组累计指针

        // nums2还有要合并的元素
        while p2 < n {
            // p1 < m 分支越界, 提前消耗完nums1
            if p1 < m && nums1[p1] > nums2[p2] {
                nums1[p] = nums1[p1]; // nums1当前值更大, 往后移动
                p1 -= 1;
            } else {
                nums1[p] = nums2[p2]; // num2当前值更大, 填入(覆盖)当前位置
                p2 -= 1;
            }
            p -= 1; // 当前位置退一步
        }

    }
}
// @lc code=end

