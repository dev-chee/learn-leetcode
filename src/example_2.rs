//! 给出两个 *非空* 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 *逆序* 的方式存储的，并且它们的每个节点只能存储 *一位* 数字。
//!
//! 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
//!
//! 您可以假设除了数字 *0* 之外，这两个数都不会以 *0* 开头。
//!
//! 示例：
//!
//! 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
//! 输出：7 -> 0 -> 8
//! 原因：342 + 465 = 807
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode-cn.com/problems/add-two-numbers
//! 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut result_next = &mut result;
    let mut l1 = &l1;
    let mut l2 = &l2;
    let mut end1 = false;
    let mut end2 = false;
    let mut carry = false;

    loop {
        let mut v1 = 0;
        let mut v2 = 0;

        if !end1 {
            match next_node(&mut l1) {
                Some(v) => {
                    v1 = v;
                }
                None => {
                    end1 = true;
                }
            }
        }

        if !end2 {
            match next_node(&mut l2) {
                Some(v) => {
                    v2 = v;
                }
                None => {
                    end2 = true;
                }
            }
        }

        if end1 && end2 {
            if carry {
                result_next = append_node(result_next, 1);
            }

            break;
        }

        let mut t = v1 + v2;
        if carry { t += 1; }

        carry = t > 9;
        if carry { t -= 10; }
        result_next = append_node(result_next, t);
    }

    result
}

fn next_node(list: &mut &Option<Box<ListNode>>) -> Option<i32> {
    let mut result = None;

    match list {
        Some(node) => {
            result = Some(node.val);
            *list = &node.next;
        }
        None => {
            result = None;
        }
    }

    result
}

fn append_node(list: &mut Option<Box<ListNode>>, val: i32) -> &mut Option<Box<ListNode>> {
    match list {
        Some(node) => {
            node.next = Some(Box::new(ListNode { val, next: None }));
            &mut node.next
        }
        None => {
            *list = Some(Box::new(ListNode { val, next: None })).clone();
            list
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_node() {
        let mut x = &Some(
            Box::new(
                ListNode {
                    val: 2,
                    next: Some(
                        Box::new(
                            ListNode {
                                val: 4,
                                next: Some(
                                    Box::new(ListNode {
                                        val: 3,
                                        next: None,
                                    })),
                            })),
                }));

        assert_eq!(next_node(&mut x), Some(2));
        assert_eq!(next_node(&mut x), Some(4));
        assert_eq!(next_node(&mut x), Some(3));
    }

    #[test]
    fn test_append_node() {
        let mut list = None;
        let mut rlist = append_node(&mut list, 1);

        rlist = append_node(rlist, 2);
        rlist = append_node(rlist, 3);

        let mut rlist = &list;

        assert_eq!(next_node(&mut rlist), Some(1));
        assert_eq!(next_node(&mut rlist), Some(2));
        assert_eq!(next_node(&mut rlist), Some(3));
    }

    #[test]
    fn test_1() {
        let x = ListNode {
            val: 2,
            next: Some(
                Box::new(
                    ListNode {
                        val: 4,
                        next: Some(
                            Box::new(
                                ListNode {
                                    val: 3,
                                    next: None,
                                })),
                    })),
        };

        let y = ListNode {
            val: 5,
            next: Some(
                Box::new(
                    ListNode {
                        val: 6,
                        next: Some(
                            Box::new(
                                ListNode {
                                    val: 4,
                                    next: None,
                                })),
                    })),
        };

        let z = ListNode {
            val: 7,
            next: Some(
                Box::new(
                    ListNode {
                        val: 0,
                        next: Some(
                            Box::new(
                                ListNode {
                                    val: 8,
                                    next: None,
                                })),
                    })),
        };

        assert_eq!(add_two_numbers(Some(Box::new(x)),
                                   Some(Box::new(y))), Some(Box::new(z)));
    }
}