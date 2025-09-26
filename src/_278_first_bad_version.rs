/*
 * @lc app=leetcode.cn id=278 lang=rust
 *
 * [278] 第一个错误的版本
 *
 * https://leetcode.cn/problems/first-bad-version/description/
 *
 * algorithms
 * Easy (46.02%)
 * Likes:    1049
 * Dislikes: 0
 * Total Accepted:    525.3K
 * Total Submissions: 1.1M
 * Testcase Example:  '5\n4'
 *
 * 
 * 你是产品经理，目前正在带领一个团队开发新的产品。不幸的是，你的产品的最新版本没有通过质量检测。由于每个版本都是基于之前的版本开发的，所以错误的版本之后的所有版本都是错的。
 * 
 * 假设你有 n 个版本 [1, 2, ..., n]，你想找出导致之后所有版本出错的第一个错误的版本。
 * 
 * 你可以通过调用 bool isBadVersion(version) 接口来判断版本号 version
 * 是否在单元测试中出错。实现一个函数来查找第一个错误的版本。你应该尽量减少对调用 API 的次数。
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 5, bad = 4
 * 输出：4
 * 解释：
 * 调用 isBadVersion(3) -> false 
 * 调用 isBadVersion(5) -> true 
 * 调用 isBadVersion(4) -> true
 * 所以，4 是第一个错误的版本。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1, bad = 1
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= bad <= n <= 2^31 - 1
 * 
 * 
 */

struct Solution;

impl Solution {
    pub fn isBadVersion(&self, idx: i32) -> bool {
        todo!()
    }
}

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        // 典型的二分查找场景: [true, ..., true, true, false, false, ..., false]
        // 按描述错误是一定存在的
        let mut low: i32 = 1;
        let mut high: i32 = n;

        while low < high {
            let mid: i32 = low + (high - low) / 2;
            let check: bool = self.isBadVersion(mid);
            if check {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}
// @lc code=end
