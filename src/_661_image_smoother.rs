/*
 * @lc app=leetcode.cn id=661 lang=rust
 *
 * [661] 图片平滑器
 *
 * https://leetcode.cn/problems/image-smoother/description/
 *
 * algorithms
 * Easy (67.00%)
 * Likes:    254
 * Dislikes: 0
 * Total Accepted:    79.8K
 * Total Submissions: 119.1K
 * Testcase Example:  '[[1,1,1],[1,0,1],[1,1,1]]'
 *
 * 图像平滑器 是大小为 3 x 3 的过滤器，用于对图像的每个单元格平滑处理，平滑处理后单元格的值为该单元格的平均灰度。
 *
 * 每个单元格的  平均灰度 定义为：该单元格自身及其周围的 8 个单元格的平均值，结果需向下取整。（即，需要计算蓝色平滑器中 9 个单元格的平均值）。
 *
 * 如果一个单元格周围存在单元格缺失的情况，则计算平均灰度时不考虑缺失的单元格（即，需要计算红色平滑器中 4 个单元格的平均值）。
 *
 *
 *
 * 给你一个表示图像灰度的 m x n 整数矩阵 img ，返回对图像的每个单元格平滑处理后的图像 。
 *
 *
 *
 * 示例 1:
 *
 *
 *
 *
 * 输入:img = [[1,1,1],[1,0,1],[1,1,1]]
 * 输出:[[0, 0, 0],[0, 0, 0], [0, 0, 0]]
 * 解释:
 * 对于点 (0,0), (0,2), (2,0), (2,2): 平均(3/4) = 平均(0.75) = 0
 * 对于点 (0,1), (1,0), (1,2), (2,1): 平均(5/6) = 平均(0.83333333) = 0
 * 对于点 (1,1): 平均(8/9) = 平均(0.88888889) = 0
 *
 *
 * 示例 2:
 *
 *
 * 输入: img = [[100,200,100],[200,50,200],[100,200,100]]
 * 输出: [[137,141,137],[141,138,141],[137,141,137]]
 * 解释:
 * 对于点 (0,0), (0,2), (2,0), (2,2): floor((100+200+200+50)/4) = floor(137.5) =
 * 137
 * 对于点 (0,1), (1,0), (1,2), (2,1): floor((200+200+50+200+100+100)/6) =
 * floor(141.666667) = 141
 * 对于点 (1,1): floor((50+200+200+200+200+100+100+100+100)/9) = floor(138.888889)
 * = 138
 *
 *
 *
 *
 * 提示:
 *
 *
 * m == img.length
 * n == img[i].length
 * 1 <= m, n <= 200
 * 0 <= img[i][j] <= 255
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 求3 * 3的平均值, 但是需要考虑越界后为0
        let m: usize = img.len();
        let n: usize = img[0].len();

        // 空矩阵
        let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                // 需要向四方位延伸1, 但是不能超出边界 [0, m - 1] [0, n - 1]
                // 需要用"饱和减法"优化 usize越界的问题
                let left: usize = i.saturating_sub(1); // 等价于: (i as i32 - 1).max(0) as usize;
                let right: usize = i.saturating_add(1).min(m - 1); // 等价于: (i as i32 + 1).min(m as i32 - 1) as usize;
                let bottom: usize = j.saturating_sub(1); // 等价于: (j as i32 - 1).max(0) as usize;
                let top: usize = j.saturating_add(1).min(n - 1); // 等价于 (j as i32 + 1).min(n as i32 - 1) as usize;

                let mut s = 0;
                let mut cnt = 0;
                for x in left..=right {
                    for y in bottom..=top {
                        s += img[x][y];
                        cnt += 1;
                    }
                }
                ans[i][j] = s / cnt;

                // TODO
                // let sum: i32 = (left..=right)
                //     .flat_map(|x| (top..=bottom).map(move |y| (x, y))) // 生成所有(x, y)元组
                //     .map(|(x, y)| img[x][y]) // 取出值
                //     .sum();
                // let cnt: i32 = ((right - left + 1) * (bottom - top + 1)) as i32; // 计算个数，更高效
            }
        }
        ans
    }
}
// @lc code=end
