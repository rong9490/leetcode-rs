/*
 * @lc app=leetcode.cn id=100 lang=rust
 *
 * [100] 相同的树
 */

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 特殊解法: PartialEq Eq --> 自动默认递归(并不是最高效)
        // TreeNode 比较字段
        // RefCell 比较内部值
        // Rc<T> 比较指向的值
        // Option Some(a) == Some(b)
        p == q

        // match (p, q) {
        //     (Some(p), Some(q)) => {
        //         let p = p.borrow(); // 取得所有权(内部可变性)
        //         let q = q.borrow();

        //         // 值相同
        //         let is_same_val: bool = p.val == q.val;
        //         // 左节点递归
        //         let is_same_left: bool = Self::is_same_tree(p.left.clone(), q.left.clone());
        //         // 右节点递归
        //         let is_same_right: bool = Self::is_same_tree(p.right.clone(), q.right.clone());
        //         return is_same_val && is_same_left && is_same_right;
        //     },
        //     (None, None) => true, // 当两个节点都为None, 表示相同
        //     _ => false, // 其余一遍为None的情况返回false
        // }
    }
}
// @lc code=end
