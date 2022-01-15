// Definition for singly-linked list.
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
}

// Version 1: This passes all the test cases, but it feels really inelegant.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let base = 10;
    let mut carry = 0;
    let mut vals: Vec<i32> = vec![];
    let mut a = &l1;
    let mut b = &l2;
    loop {
        let mut a_val = 0;
        let mut b_val = 0;
        match (a, b) {
            (None, None) => {
                if carry == 0 {
                    break;
                }
            }
            (Some(aa), None) => {
                a = &aa.next;
                a_val = aa.val;
            }
            (None, Some(bb)) => {
                b = &bb.next;
                b_val = bb.val;
            }
            (Some(aa), Some(bb)) => {
                a = &aa.next;
                a_val = aa.val;
                b = &bb.next;
                b_val = bb.val;
            }
        }
        let sum = a_val + b_val + carry;
        carry = sum / base;
        vals.push(sum % base);
    }
    let mut result: Option<Box<ListNode>> = None;
    for val in vals.iter().rev() {
        match result {
            None => result = Some(Box::new(ListNode::new(*val))),
            Some(x) => {
                result = Some(Box::new(ListNode {
                    val: *val,
                    next: Some(x),
                }))
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        for val in vals.iter().rev() {
            match result {
                None => result = Some(Box::new(ListNode::new(*val))),
                Some(x) => {
                    result = Some(Box::new(ListNode {
                        val: *val,
                        next: Some(x),
                    }))
                }
            }
        }
        result
    }

    #[test]
    fn addtwonumbers_01() {
        let l1 = from_vec(vec![2, 4, 3]);
        let l2 = from_vec(vec![5, 6, 4]);
        let result = from_vec(vec![7, 0, 8]);
        assert_eq!(result, add_two_numbers(l1, l2));
    }

    #[test]
    fn addtwonumbers_02() {
        let l1 = from_vec(vec![0]);
        let l2 = from_vec(vec![0]);
        let result = from_vec(vec![0]);
        assert_eq!(result, add_two_numbers(l1, l2));
    }

    #[test]
    fn addtwonumbers_03() {
        let l1 = from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = from_vec(vec![9, 9, 9, 9]);
        let result = from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(result, add_two_numbers(l1, l2));
    }

    #[test]
    fn addtwonumbers_04() {
        let l1 = from_vec(vec![9]);
        let l2 = from_vec(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let result = from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, add_two_numbers(l1, l2));
    }
}
