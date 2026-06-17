
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

 pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;
    
    while list1.is_some() && list2.is_some() {
        let l1_val = list1.as_ref().unwrap().val;
        let l2_val = list2.as_ref().unwrap().val;
        
        if l1_val <= l2_val {
            tail.next = list1.take();
            list1 = tail.next.as_mut().unwrap().next.take();
        } else {
            tail.next = list2.take();
            list2 = tail.next.as_mut().unwrap().next.take();
        }
        
        tail = tail.next.as_mut().unwrap();
    }
    
    tail.next = if list1.is_some() { list1 } else { list2 };
    
    dummy.next
}
}