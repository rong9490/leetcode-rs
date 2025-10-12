/*
 * @lc app=leetcode.cn id=1207 lang=rust
 *
 * [1207] 独一无二的出现次数
 *
 * https://leetcode.cn/problems/unique-number-of-occurrences/description/
 *
 * algorithms
 * Easy (74.42%)
 * Likes:    245
 * Dislikes: 0
 * Total Accepted:    123.4K
 * Total Submissions: 165.8K
 * Testcase Example:  '[1,2,2,1,1,3]'
 *
 * 给你一个整数数组 arr，如果每个数的出现次数都是独一无二的，就返回 true；否则返回 false。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：arr = [1,2,2,1,1,3]
 * 输出：true
 * 解释：在该数组中，1 出现了 3 次，2 出现了 2 次，3 只出现了 1 次。没有两个数的出现次数相同。
 * 
 * 示例 2：
 * 
 * 
 * 输入：arr = [1,2]
 * 输出：false
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：arr = [-3,0,1,-3,1,1,1,-3,10,0]
 * 输出：true
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= arr.length <= 1000
 * -1000 <= arr[i] <= 1000
 * 
 * 
 */

struct Solution;

// 工具方法库 itertools

// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // 用hashMap登录每个值的出现次数+hashSet判断重复次数
        // let mut cnt: HashMap<&i32, i32> = HashMap::new();
        // arr.iter().for_each(|x| *cnt.entry(x).or_insert(0) += 1);

        // 迭代器写法, 分别产生 map和set

        // iter()转为迭代器: fold累计 + HashMap::new()初始化
        let cnt_map: HashMap<&i32, i32> = arr.iter().fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });
        let cnt_set: HashSet<&i32> = cnt_map.values().collect::<HashSet<&i32>>();

        // set去重, 判断是否存在重复的次数
        cnt_map.len() == cnt_set.len()
    }
}
// @lc code=end

