/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 *
 * https://leetcode.cn/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (67.90%)
 * Likes:    3791
 * Dislikes: 0
 * Total Accepted:    2.1M
 * Total Submissions: 3.1M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：l1 = [1,2,4], l2 = [1,3,4]
 * 输出：[1,1,2,3,4,4]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：l1 = [], l2 = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：l1 = [], l2 = [0]
 * 输出：[0]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 两个链表的节点数目范围是 [0, 50]
 * -100 
 * l1 和 l2 均按 非递减顺序 排列
 * 
 * 
 */

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// @lc code=start
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy: ListNode = ListNode::new(0); // 哨兵节点!
        let mut cur: &mut ListNode = &mut dummy; // 链表末尾指针, 初始化

        // 同时判断两个链表, 并分别取出一个节点
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            let mut list: &mut Option<Box<ListNode>> = if node1.val < node2.val { &mut list1 } else { &mut list2 };
            cur.next = list.take(); // Optionn.take取出节点; 并追加
            cur = cur.next.as_mut()?; // 往后移动一步
            *list = cur.next.take(); // 原链表也需要往后移动一步
        }
        // 当一边链表耗尽, 需要拼接另一边
        cur.next = list1.or(list2); // Option.or布尔运算
        dummy.next // 返回最终的新链表
    }
}
// @lc code=end

