/*
 * @lc app=leetcode.cn id=1769 lang=rust
 *
 * [1769] 移动所有球到每个盒子所需的最小操作数
 *
 * https://leetcode.cn/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/
 *
 * algorithms
 * Medium (87.66%)
 * Likes:    141
 * Dislikes: 0
 * Total Accepted:    45.5K
 * Total Submissions: 52K
 * Testcase Example:  '"110"'
 *
 * 有 n 个盒子。给你一个长度为 n 的二进制字符串 boxes ，其中 boxes[i] 的值为 '0' 表示第 i 个盒子是 空 的，而
 * boxes[i] 的值为 '1' 表示盒子里有 一个 小球。
 *
 * 在一步操作中，你可以将 一个 小球从某个盒子移动到一个与之相邻的盒子中。第 i 个盒子和第 j 个盒子相邻需满足 abs(i - j) == 1
 * 。注意，操作执行后，某些盒子中可能会存在不止一个小球。
 *
 * 返回一个长度为 n 的数组 answer ，其中 answer[i] 是将所有小球移动到第 i 个盒子所需的 最小 操作数。
 *
 * 每个 answer[i] 都需要根据盒子的 初始状态 进行计算。
 *
 *
 *
 * 示例 1：
 *
 * 输入：boxes = "110"
 * 输出：[1,1,3]
 * 解释：每个盒子对应的最小操作数如下：
 * 1) 第 1 个盒子：将一个小球从第 2 个盒子移动到第 1 个盒子，需要 1 步操作。
 * 2) 第 2 个盒子：将一个小球从第 1 个盒子移动到第 2 个盒子，需要 1 步操作。
 * 3) 第 3 个盒子：将一个小球从第 1 个盒子移动到第 3 个盒子，需要 2 步操作。将一个小球从第 2 个盒子移动到第 3 个盒子，需要 1
 * 步操作。共计 3 步操作。
 *
 *
 * 示例 2：
 *
 * 输入：boxes = "001011"
 * 输出：[11,8,5,4,3,4]
 *
 *
 *
 * 提示：
 *
 *
 * n == boxes.length
 * 1 <= n <= 2000
 * boxes[i] 为 '0' 或 '1'
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    // 需要遍历每一个位置; 或者DP
    pub fn min_operations(boxes: String) -> Vec<i32> {
        // // 字符串转为对应的数字
        // let boxes: Vec<i32> = boxes.bytes().map(|x| (x - b'0') as i32).collect();
        // let mut cur: i32 = boxes.iter().enumerate().map(|(i, x)| (i as i32) * x).sum();
        // let total: i32 = boxes.iter().filter(|&&x| x == 1).sum();
        // let mut ans: Vec<i32> = Vec::with_capacity(boxes.len());
        // let mut count: i32 = 0;
        // for x in boxes.into_iter() {
        //     ans.push(cur);
        //     if x == 1 {
        //         count += 1;
        //     }
        //     // 两端的距离计算
        //     cur += 2 * count - total; // cur + count - (total-count)
        // }
        // return ans;

        // TODO
        let (mut step, mut cnt) = (0, 0);
        let mut ans: Vec<i32> = boxes
            .as_bytes()
            .iter()
            .enumerate()
            .scan(0i32, |state, (idx, &b)| {
                *state = *state + (cnt << 1);
                if b == b'1' {
                    step += idx + 1;
                    cnt += 1;
                }
                Some(*state)
            })
            .collect();
        ans.iter_mut()
            .enumerate()
            .for_each(|(idx, x)| *x = *x + step as i32 - cnt * (idx as i32 + 1));
        ans
    }
}
// @lc code=end
