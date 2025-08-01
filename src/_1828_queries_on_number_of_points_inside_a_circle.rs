/*
 * @lc app=leetcode.cn id=1828 lang=rust
 *
 * [1828] 统计一个圆中点的数目
 *
 * https://leetcode.cn/problems/queries-on-number-of-points-inside-a-circle/description/
 *
 * algorithms
 * Medium (87.79%)
 * Likes:    80
 * Dislikes: 0
 * Total Accepted:    34.3K
 * Total Submissions: 39.1K
 * Testcase Example:  '[[1,3],[3,3],[5,3],[2,2]]\n[[2,3,1],[4,3,1],[1,1,2]]'
 *
 * 给你一个数组 points ，其中 points[i] = [xi, yi] ，表示第 i 个点在二维平面上的坐标。多个点可能会有 相同 的坐标。
 *
 * 同时给你一个数组 queries ，其中 queries[j] = [xj, yj, rj] ，表示一个圆心在 (xj, yj) 且半径为 rj
 * 的圆。
 *
 * 对于每一个查询 queries[j] ，计算在第 j 个圆 内 点的数目。如果一个点在圆的 边界上 ，我们同样认为它在圆 内 。
 *
 * 请你返回一个数组 answer ，其中 answer[j]是第 j 个查询的答案。
 *
 *
 *
 * 示例 1：
 *
 * 输入：points = [[1,3],[3,3],[5,3],[2,2]], queries = [[2,3,1],[4,3,1],[1,1,2]]
 * 输出：[3,2,2]
 * 解释：所有的点和圆如上图所示。
 * queries[0] 是绿色的圆，queries[1] 是红色的圆，queries[2] 是蓝色的圆。
 *
 *
 * 示例 2：
 *
 * 输入：points = [[1,1],[2,2],[3,3],[4,4],[5,5]], queries =
 * [[1,2,2],[2,2,2],[4,3,2],[4,3,3]]
 * 输出：[2,3,2,4]
 * 解释：所有的点和圆如上图所示。
 * queries[0] 是绿色的圆，queries[1] 是红色的圆，queries[2] 是蓝色的圆，queries[3] 是紫色的圆。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= points.length <= 500
 * points[i].length == 2
 * 0 <= x​​​​​​i, y​​​​​​i <= 500
 * 1 <= queries.length <= 500
 * queries[j].length == 3
 * 0 <= xj, yj <= 500
 * 1 <= rj <= 500
 * 所有的坐标都是整数。
 *
 *
 */

pub struct Solution;

// @lc code=start
// use std::num;
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // let mut ret: Vec<i32> = vec![0; queries.len()]; // 初始值为0, 判断所有的圆的结果
        // // 遍历圆圈
        // for i in 0..queries.len() {
        //     // 圆心: x, y, r
        //     let (x, y, r) = (queries[i][0], queries[i][1], queries[i][2]);
        //     // 遍历点, 判断是否在圆内 --> 欧式距离
        //     for point in points.iter() {
        //         // 如何便捷求出平方?
        //         let distance: i32 = (point[0] - x) * (point[0] - x) + (point[1] - y) * (point[1] - y);
        //         ret[i] += if distance <= r * r { 1 } else { 0 };
        //     }
        // }
        // ret

        // 优化成迭代器
        let que_iter = queries.iter();

        return que_iter.map(|x| {
            // 过滤在圈内的点, 再计算个数
            let inner_points = points
                .iter()
                .filter(|p| (p[0] - x[0]).pow(2) + (p[1] - x[1]).pow(2) <= x[2] * x[2]);
            return inner_points.count() as i32;
        }).collect(); // collect 将迭代器转回为 vec容器
    }
}
// @lc code=end
