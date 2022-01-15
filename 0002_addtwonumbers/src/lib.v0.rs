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

// Version 0:
// The examples used for verification used values that exceed i32, so this solution is not valid
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let a = from_listnode(l1);
    let b = from_listnode(l2);
    to_listnode(a + b)
}


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

fn from_listnode(ln: Option<Box<ListNode>>) -> i32 {
    let mut pow = 0;
    let mut sum = 0;
    let mut node = ln;
    loop {
        match node {
            None => break,
            Some(x) => {
                sum += x.val * i32::pow(10, pow);
                pow += 1;
                node = x.next;
            }
        }
    }
    sum
}

fn to_listnode(a: i32) -> Option<Box<ListNode>> {
    let modulus = 10;
    let mut total = a;
    let mut vals: Vec<i32> = vec![];
    loop {
        // optionally, convert to string, then peel off the characters and convert back to i32.
        vals.push(total % modulus);
        total /= modulus; // shift
        if total == 0 {
            break from_vec(vals.iter().map(|x| *x).collect::<Vec<i32>>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
