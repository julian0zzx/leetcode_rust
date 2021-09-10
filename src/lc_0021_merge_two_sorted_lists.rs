
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    pub fn link(&mut self, next_node: ListNode) { // same as self.next = Some(Box::new(?))
        let nn = Some(Box::new(next_node));
        self.next = nn;
    }
}

impl ListNode {
    // sorted input, in ascending order
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}

impl ListNode {
    fn get_next(self) -> Option<Box<ListNode>> {
        self.next
    }
}

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // if None == l1 {
    //     return l2;
    // } else if None == l2 {
    //     return l1;
    // } else {
    //     let v1n = l1.unwrap();
    //     let v1 = v1n.val;
    //     let v2n = l2.unwrap();
    //     let v2 = v2n.val;
    //     let vv = if v1 <= v2 { v1 } else { v2 };
    //     let mut rl = ListNode::new(vv);
    //     let mut v1_node = v1n.get_next();
    //     let mut v2_node = v2n.get_next();
    //     loop {
    //         if v1_node.is_none() && v2_node.is_none() {
    //             break;
    //         }

    //         let mut v1 = 0;
    //         if v1_node.is_none() {
    //             v1 = (*v1_node.unwrap()).val;
    //         } else {
    //             v1_node = (*v1_node.unwrap()).get_next();
    //         }
    //         let mut v2 = 0;
    //         if v2_node.is_none() {
    //             v2 = v2_node.unwrap().val;
    //         } else {
    //             v2_node = (*v2_node.unwrap()).get_next();
    //         }

    //         let vv = if v1 <= v2 { v1 } else { v2 };
    //         rl.next = Some(Box::new(ListNode::new(vv)));
    //     }
    //     return Some(Box::new(rl));
    // }
    None
}

#[cfg(test)]
mod merge_two_lists_test {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        // let mut l7 = ListNode::new(7);
        // l7.next = Some(Box::new(ListNode::new(5))); // 75
        // let mut l5 = ListNode::new(4);
        // l5.next = Some(Box::new(ListNode::new(2))); // 42
        let mut l7 = ListNode::new(5);
        l7.next = Some(Box::new(ListNode::new(7))); // 57
        let mut l5 = ListNode::new(2);
        l5.next = Some(Box::new(ListNode::new(4))); // 24
    
        let mut ll = ListNode::new(2);
        let mut ll4 = ListNode::new(4);
        let mut ll5 = ListNode::new(5);
        let ll7 = ListNode::new(7);
        ll5.next = Some(Box::new(ll7));
        ll4.next = Some(Box::new(ll5));
        ll.next = Some(Box::new(ll4));

        assert_eq!(Some(Box::new(ll)), ListNode::merge_two_lists(Some(Box::new(l5)), Some(Box::new(l7))));
    }
}
