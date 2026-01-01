/*
 * @lc app=leetcode.cn id=3074 lang=rust
 *
 * [3074] 重新分装苹果
 *
 * https://leetcode.cn/problems/apple-redistribution-into-boxes/description/
 *
 * algorithms
 * Easy (72.36%)
 * Likes:    34
 * Dislikes: 0
 * Total Accepted:    26.7K
 * Total Submissions: 35.2K
 * Testcase Example:  '[1,3,2]\n[4,3,1,5,2]'
 *
 * 给你一个长度为 n 的数组 apple 和另一个长度为 m 的数组 capacity 。
 * 
 * 一共有 n 个包裹，其中第 i 个包裹中装着 apple[i] 个苹果。同时，还有 m 个箱子，第 i 个箱子的容量为 capacity[i]
 * 个苹果。
 * 
 * 请你选择一些箱子来将这 n 个包裹中的苹果重新分装到箱子中，返回你需要选择的箱子的 最小 数量。
 * 
 * 注意，同一个包裹中的苹果可以分装到不同的箱子中。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：apple = [1,3,2], capacity = [4,3,1,5,2]
 * 输出：2
 * 解释：使用容量为 4 和 5 的箱子。
 * 总容量大于或等于苹果的总数，所以可以完成重新分装。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：apple = [5,5,5], capacity = [2,4,2,7]
 * 输出：4
 * 解释：需要使用所有箱子。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n == apple.length <= 50
 * 1 <= m == capacity.length <= 50
 * 1 <= apple[i], capacity[i] <= 50
 * 输入数据保证可以将包裹中的苹果重新分装到箱子中。
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        // 分苹果 -> 由于可以完全打散 -> 求和+排序+贪心, 减去容量
        let mut sum: i32 = apple.iter().sum::<i32>();
        // 倒序(也就是由大到小, 先消耗大箱子)
        capacity.sort_unstable_by_key(|cap| -cap);
        for (i, size) in capacity.iter().enumerate() {
            sum -= size;
            // 已经够装了
            if sum <= 0 {
                return (i + 1) as i32 // 共使用的箱子个数
            }
        }
        unreachable!()
    }
}
// @lc code=end

