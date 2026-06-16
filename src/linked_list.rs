
// Definition for singly-linked list (LeetCode provides this).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{

        let mut prev: Option<Box<ListNode>> = None;

        let mut curr = head;

        while let Some(mut node) = curr {

            let  next = node.next.take();

            node.next = prev;

            prev = Some(node);

            curr = next;
        }

        prev
    }
}