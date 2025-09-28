/*
 * @lc app=leetcode.cn id=812 lang=rust
 *
 * [812] 最大三角形面积
 *
 * https://leetcode.cn/problems/largest-triangle-area/description/
 *
 * algorithms
 * Easy (68.25%)
 * Likes:    219
 * Dislikes: 0
 * Total Accepted:    46.5K
 * Total Submissions: 67.1K
 * Testcase Example:  '[[0,0],[0,1],[1,0],[0,2],[2,0]]'
 *
 * 给你一个由 X-Y 平面上的点组成的数组 points ，其中 points[i] = [xi, yi]
 * 。从其中取任意三个不同的点组成三角形，返回能组成的最大三角形的面积。与真实值误差在 10^-5 内的答案将会视为正确答案。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
 * 输出：2.00000
 * 解释：输入中的 5 个点如上图所示，红色的三角形面积最大。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：points = [[1,0],[0,0],[0,1]]
 * 输出：0.50000
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 3 <= points.length <= 50
 * -50 <= xi, yi <= 50
 * 给出的所有点 互不相同
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {

    // 求面积方法
    pub fn area(points: Vec<Vec<i32>>) -> f64 {
        let a: &Vec<i32> = &points[0];
        let b: &Vec<i32> = &points[0];
        let c: &Vec<i32> = &points[0];
        
        (x_1 * y_2 + x_2 * y_3 + x_3 * y_1 - x_1 * y_3 - x_2 * y_1 - x_3 * y_2).abs() as f64 * 0.5f64


    }

    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        
    }
}
// @lc code=end


// let mut area = f64::MIN;
// for (i, p1) in points.iter().enumerate() {
//     for (j, p2) in points[i + 1..].iter().enumerate() {
//         for p3 in points[j + 1..].iter() {
//             if let [x1, y1] = p1[..] { if let [x2, y2] = p2[..] { if let [x3, y3] = p3[..] { area = area.max(0.5 * ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs() as f64); } } }
//         }
//     }
// }
// area
