/*
 * @lc app=leetcode.cn id=3461 lang=rust
 *
 * [3461] 判断操作后字符串中的数字是否相等 I
 *
 * https://leetcode.cn/problems/check-if-digits-are-equal-in-string-after-operations-i/description/
 *
 * algorithms
 * Easy (75.50%)
 * Likes:    13
 * Dislikes: 0
 * Total Accepted:    13.9K
 * Total Submissions: 17.7K
 * Testcase Example:  '"3902"'
 *
 * 给你一个由数字组成的字符串 s 。重复执行以下操作，直到字符串恰好包含 两个 数字：
 *
 *
 * 从第一个数字开始，对于 s 中的每一对连续数字，计算这两个数字的和 模 10。
 * 用计算得到的新数字依次替换 s 的每一个字符，并保持原本的顺序。
 *
 *
 * 如果 s 最后剩下的两个数字 相同 ，返回 true 。否则，返回 false。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入： s = "3902"
 *
 * 输出： true
 *
 * 解释：
 *
 *
 * 一开始，s = "3902"
 * 第一次操作：
 *
 * (s[0] + s[1]) % 10 = (3 + 9) % 10 = 2
 * (s[1] + s[2]) % 10 = (9 + 0) % 10 = 9
 * (s[2] + s[3]) % 10 = (0 + 2) % 10 = 2
 * s 变为 "292"
 *
 *
 * 第二次操作：
 *
 * (s[0] + s[1]) % 10 = (2 + 9) % 10 = 1
 * (s[1] + s[2]) % 10 = (9 + 2) % 10 = 1
 * s 变为 "11"
 *
 *
 * 由于 "11" 中的数字相同，输出为 true。
 *
 *
 *
 * 示例 2：
 *
 *
 * 输入： s = "34789"
 *
 * 输出： false
 *
 * 解释：
 *
 *
 * 一开始，s = "34789"。
 * 第一次操作后，s = "7157"。
 * 第二次操作后，s = "862"。
 * 第三次操作后，s = "48"。
 * 由于 '4' != '8'，输出为 false。
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= s.length <= 100
 * s 仅由数字组成。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        // let mut os: String = s;
        // // 循环模拟计算过程
        // while os.len() > 2 {
        //     let mut new_s: String = String::new();
        //     let mut i: usize = 0;
        //     while i < os.len() - 1 {
        //         let first: i8 = os[i..=i].parse::<i8>().unwrap();
        //         let second: i8 = os[i + 1..=i + 1].parse::<i8>().unwrap();
        //         // HINT 理解这两个错
        //         // 临时值的引用会报错! 因为其立即会被丢弃, 形成悬垂指针
        //         // let append_str: &str = ((first + second) % 10).to_string().as_ref(); // String -> &str
        //         // 生命周期不够长, 也会报错
        //         // let append_str: &str = {
        //         //     let sum: i8 = (first + second) % 10;
        //         //     let s: String = sum.to_string();
        //         //     s.as_str() // 合法，因为 s 的生命周期持续到这个代码块结束
        //         // };

        //         // 正确写法, 拆分3行分别赋值
        //         let sum: i8 = (first + second) % 10;
        //         let sum_str: String = sum.to_string(); // String，拥有所有权
        //         let append_str: &str = sum_str.as_str(); // &str，合法 ✅
        //         new_s += append_str;
        //         i += 1;
        //     }
        //     os = new_s;
        // }

        // os[0..=0] == os[1..=1]

        // 迭代器写法: 滑动窗口, 循环压缩字符容器
        fn compress(chars: &[char]) -> Vec<char> {
            chars
                // 相邻字符组成数组['1', '2']
                .windows(2) 
                .map(|pair| -> char {
                    // 可以用parse解析, 也可以用to_digit转数位
                    let first: u32 = pair[0].to_digit(10).unwrap(); // char -> digit
                    let second: u32 = pair[1].to_digit(10).unwrap();
                    let sum: u32 = (first + second) % 10;
                    char::from_digit(sum, 10).unwrap() // digit -> char
                })
                .collect::<Vec<char>>() // 收集成 Vec<char>
        }

        // while可以改为递归写法; s.chars() 字符容器
        let mut scan_chars: Vec<char> = s.chars().collect::<Vec<char>>();
        while scan_chars.len() > 2 {
            scan_chars = compress(&scan_chars);
        }
        scan_chars.len() == 2 && scan_chars[0] == scan_chars[1]
    }
}
// @lc code=end
