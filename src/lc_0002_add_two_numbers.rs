// Definition for singly-linked list
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

    pub fn link(&mut self, next_node: ListNode) { // same as self.next = Some(Box::new(?))
        let nn = Some(Box::new(next_node));
        self.next = nn;
    }
}

impl ListNode {
    fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }
}


pub fn add_two_numbers_f(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    ListNode::add_two_numbers_co(l1, l2, 0)

}


impl ListNode {
    pub fn add_two_numbers_co(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carryover: i32) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(l), Some(r)) => {
                todo!("======= illegal logical =======");
                let nv = l.val + r.val + carryover;
                Some(Box::new(ListNode{val: nv %10, next: Self::add_two_numbers_co(l.next, r.next, nv / 10)}))
            }
        }
    }
}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if None == l1 && None == l2 {
        return Some(Box::new(ListNode::new(0)));
    } else if None == l1 && None != l2 {
        return l2;
    } else if None != l1 && None == l2 {
        return l1;
    } else {
        let mut l1ln : ListNode = *l1.unwrap();
        let mut l2ln : ListNode = *l2.unwrap();
        
        let l1l2v = l1ln.val + l2ln.val;
        let mut carry_over = if l1l2v > 9 { 1 } else { 0 };
        let mut first_n  = ListNode::new(l1l2v % 10);
        let mut cur_n = first_n.get_last();
        loop {
            if None != l1ln.next || None != l2ln.next {
                l1ln = if None != l1ln.next { *l1ln.next.unwrap() } else {ListNode::new(0)};
                l2ln =  if None != l2ln.next { *l2ln.next.unwrap() } else {ListNode::new(0)};
                let l1l2v = l1ln.val + l2ln.val + carry_over;
                carry_over = if l1l2v > 9 { 1 } else { 0 };
                let n = ListNode::new(l1l2v % 10);
                cur_n.next = Some(Box::new(n));
                cur_n = cur_n.get_last();
            } else {
                if 0 != carry_over {
                    if None != cur_n.next {
                        let mut crnn = cur_n;
                        let mut nn = ListNode::new(crnn.val);
                        nn.next = Some(Box::new(ListNode::new(1)));
                        crnn.next = Some(Box::new(nn));
                    } else {
                        cur_n.next = Some(Box::new(ListNode::new(1)));
                    }
                }
                break;
            }
        }
        return Some(Box::new(first_n));
    }
}

#[cfg(test)]
mod add_two_numbers_test {
    use super::*;

    #[test]
    fn test_add_two_numbers_12() {
        let l7 = ListNode::new(7);
        let l5 = ListNode::new(5);
        let mut l21 = ListNode::new(2);
        let l21_1 = ListNode::new(1);
        l21.link(l21_1);
        assert_eq!(l21, *add_two_numbers(Some(Box::new(l7)), Some(Box::new(l5))).unwrap(), "7 + 5 = 12");
    }

    #[test]
    fn test_add_two_numbers_81() {
        let mut l7 = ListNode::new(7);
        l7.next = Some(Box::new(ListNode::new(5))); // 57
        let mut l5 = ListNode::new(4);
        l5.next = Some(Box::new(ListNode::new(2))); // 24
        let mut l21 = ListNode::new(1);
        let l21_1 = ListNode::new(8); // 81
        l21.link(l21_1);
        assert_eq!(l21, *add_two_numbers(Some(Box::new(l7)), Some(Box::new(l5))).unwrap(), "57 + 24 = 81");
    }

    #[test]
    fn test_add_two_numbers_103() {
        let mut l7 = ListNode::new(7);
        l7.next = Some(Box::new(ListNode::new(6))); // 67
        let mut l5 = ListNode::new(6);
        l5.next = Some(Box::new(ListNode::new(3))); // 36
        let mut l103 = ListNode::new(3); // 103 
        let mut l1030 = ListNode::new(0);
        l1030.link(ListNode::new(1));
        l103.link(l1030);
        assert_eq!(l103, *add_two_numbers(Some(Box::new(l7)), Some(Box::new(l5))).unwrap(), "67 + 36 = 103");
    }

    #[test]
    fn test_add_two_numbers_807() {
        let mut l342 = ListNode::new(2); // 342 
        let mut l342_4 = ListNode::new(4);
        l342_4.link(ListNode::new(3));
        l342.link(l342_4);

        let mut l465 = ListNode::new(5); // 465 
        let mut l465_6 = ListNode::new(6);
        l465_6.link(ListNode::new(4));
        l465.link(l465_6);

        let mut l807 = ListNode::new(7); // 807
        let mut l807_0 = ListNode::new(0);
        l807_0.next = Some(Box::new(ListNode::new(8)));
        l807.next = Some(Box::new(l807_0));
        
        assert_eq!(l807, *add_two_numbers(Some(Box::new(l342)), Some(Box::new(l465))).unwrap(), "342 + 465 = 807");
    }


    #[test]
    fn test_add_two_numbers_553() {
        let mut l47 = ListNode::new(7);
        l47.next = Some(Box::new(ListNode::new(4))); // 47
        let l6 = ListNode::new(6);
        let mut l103 = ListNode::new(3); // 53 
        l103.link(ListNode::new(5));
        assert_eq!(l103, *add_two_numbers(Some(Box::new(l47)), Some(Box::new(l6))).unwrap(), "47 + 6 = 53");
    }

    #[test]
    fn test_add_two_numbers_108() {
        let mut l99 = ListNode::new(9);
        l99.next = Some(Box::new(ListNode::new(9))); // 99
        let l9 = ListNode::new(9); 
        let mut l108 = ListNode::new(8); // 108
        let mut l108_0 = ListNode::new(0);
        l108_0.next = Some(Box::new(ListNode::new(1)));
        l108.next = Some(Box::new(l108_0));

        assert_eq!(l108, *add_two_numbers(Some(Box::new(l99)), Some(Box::new(l9))).unwrap(), "47 + 6 = 53");
    }

    #[test]
    fn test_add_two_numbers_1008() {
        let mut l999 = ListNode::new(9);
        let mut l999_9 = ListNode::new(9);
        l999_9.next = Some(Box::new(ListNode::new(9)));
        l999.next = Some(Box::new(l999_9)); // 999
        let l9 = ListNode::new(9); 
        let mut l1008 = ListNode::new(8); // 1008
        let mut l1008_0 = ListNode::new(0);
        let mut l1008_00 = ListNode::new(0);
        l1008_00.next = Some(Box::new(ListNode::new(1)));
        l1008_0.next = Some(Box::new(l1008_00));
        l1008.next = Some(Box::new(l1008_0));
        println!("{:?}", l999);
        println!("{:?}", l1008);
        assert_eq!(l1008, *add_two_numbers(Some(Box::new(l999)), Some(Box::new(l9))).unwrap(), "999 + 9 = 1008");
    }

}
