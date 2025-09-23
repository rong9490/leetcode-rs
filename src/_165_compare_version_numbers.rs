/*
 * @lc app=leetcode.cn id=165 lang=rust
 *
 * [165] 比较版本号
 *
 * https://leetcode.cn/problems/compare-version-numbers/description/
 *
 * algorithms
 * Medium (54.16%)
 * Likes:    475
 * Dislikes: 0
 * Total Accepted:    227.2K
 * Total Submissions: 414.7K
 * Testcase Example:  '"1.2"\n"1.10"'
 *
 * 给你两个 版本号字符串 version1 和 version2 ，请你比较它们。版本号由被点 '.' 分开的修订号组成。修订号的值 是它 转换为整数
 * 并忽略前导零。
 *
 * 比较版本号时，请按 从左到右的顺序 依次比较它们的修订号。如果其中一个版本字符串的修订号较少，则将缺失的修订号视为 0。
 *
 * 返回规则如下：
 *
 *
 * 如果 version1 < version2 返回 -1，
 * 如果 version1 > version2 返回 1，
 * 除此之外返回 0。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：version1 = "1.2", version2 = "1.10"
 *
 * 输出：-1
 *
 * 解释：
 *
 * version1 的第二个修订号为 "2"，version2 的第二个修订号为 "10"：2 < 10，所以 version1 <
 * version2。
 *
 *
 * 示例 2：
 *
 *
 * 输入：version1 = "1.01", version2 = "1.001"
 *
 * 输出：0
 *
 * 解释：
 *
 * 忽略前导零，"01" 和 "001" 都代表相同的整数 "1"。
 *
 *
 * 示例 3：
 *
 *
 * 输入：version1 = "1.0", version2 = "1.0.0.0"
 *
 * 输出：0
 *
 * 解释：
 *
 * version1 有更少的修订号，每个缺失的修订号按 "0" 处理。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= version1.length, version2.length <= 500
 * version1 和 version2 仅包含数字和 '.'
 * version1 和 version2 都是 有效版本号
 * version1 和 version2 的所有修订号都可以存储在 32 位整数 中
 *
 *
 */

struct Solution;

// @lc code=start
use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        // // 版本号字符串按'.'拆分段 Vec<&str>
        // let v1_vec: Vec<&str> = version1.split('.').collect::<Vec<&str>>();
        // let v2_vec: Vec<&str> = version2.split('.').collect::<Vec<&str>>();

        // // 从高到低最多三段, 但是可能缺失
        // let size1: usize = v1_vec.len();
        // let size2: usize = v2_vec.len();
        // let max_size: usize = size1.max(size2);

        // for i in 0..max_size {
        //     // 每一段解析为数字, 如果没有则默认值为0
        //     let v1_num: i32 = if i < size1 { v1_vec[i].parse::<i32>().unwrap() } else { 0 };
        //     let v2_num: i32 = if i < size2 { v2_vec[i].parse::<i32>().unwrap() } else { 0 };

        //     // 如果相等则跳过进入下一段, 否则直接比较
        //     if v1_num != v2_num {
        //         return if v1_num < v2_num { -1 } else { 1 };
        //     }
        // }
        // 全等
        // 0

        // 转为迭代器
        let v1_segments = version1.split('.');
        let v2_segments = version2.split('.');
        let mut v1_iter = v1_segments.map(|s: &str| s.parse::<i32>().unwrap_or(0));
        let mut v2_iter = v2_segments.map(|s: &str| s.parse::<i32>().unwrap_or(0));

        // ✅ 改造成loop O(n)
        loop {
            let v1: Option<i32> = v1_iter.next();
            let v2: Option<i32> = v2_iter.next();
            if v1.is_none() && v2.is_none() {
                return 0; // 边界情况
            }
            let v1_num: i32 = v1.unwrap_or(0);
            let v2_num: i32 = v2.unwrap_or(0);

            match v1_num.cmp(&v2_num) {
                Less => return -1,
                Greater => return 1,
                Equal => {} // 下一段
            }
        }

        unreachable!();
    }
}
// @lc code=end
