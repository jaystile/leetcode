pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // not sure if a clone is needed here, but we want a mutable vector.
    let mut nums = nums.clone(); 
    loop {    
        // start with the last index to prevent index collisions
        let idx = nums.len() - 1;

        // should really check index safety
        let idx_val = nums.pop().expect("There should have been a value to extract");
        match nums.iter().enumerate().find(|(_i,x)| *x + idx_val == target) {
            Some((i,_)) => { return vec![i as i32,idx as i32]; },
            None => { /* NOOP */ },
        }
    }
}

pub fn two_sum_02(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Pre-filtering any values that exceed the target to reduce the search space
    // negative numbers in the search space, indicates that filtering is difficult
    // as really large number can be cancelled by really large negative numbers
    let mut enumerations = nums.iter().enumerate().collect::<Vec<(usize,&i32)>>();
    loop {    
        // should really check index safety
        let last = enumerations.pop().expect("There should have been a value to extract");
        match enumerations.iter().find(|(_i,&x)| x + last.1 == target) {
            Some((i,_)) => { return vec![*i as i32, last.0 as i32]; },
            None => { /* NOOP */ },
        }
    }
}

use std::collections::HashMap;
pub fn two_sum_03(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Hash all the numbers and store their index in a HashMap
    let mut map = HashMap::new();
    for (idx,val) in nums.iter().enumerate() {
        if map.contains_key(&(target - val)) {
            let idx1 = idx as i32;
            let idx2 = *map.get(&(target-val)).unwrap();
            return vec!(std::cmp::min(idx1,idx2), std::cmp::max(idx1,idx2));
        }
        map.entry(*val).or_insert(idx as i32); 
    }
    panic!("Should not have reached this point");
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_two_sum_01() {
        let result = two_sum(vec![2,7,11,15],9);
        assert_eq!(result, vec![0, 1]);
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
        let result = two_sum(vec![2,7,11,15],9);
        assert_eq!(result, vec![0, 1]);
        let result = two_sum(vec![0, 4, 3, 0], 0);
        assert_eq!(result, vec![0, 3]);
        let result = two_sum(vec![-3,4,3,90], 0);
        assert_eq!(result, vec![0,2]);
    }

    #[test]
    fn test_two_sum_02() {
        let result = two_sum_02(vec![2,7,11,15],9);
        assert_eq!(result, vec![0, 1]);
        let result = two_sum_02(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
        let result = two_sum_02(vec![2,7,11,15],9);
        assert_eq!(result, vec![0, 1]);
        let result = two_sum_02(vec![0, 4, 3, 0], 0);
        assert_eq!(result, vec![0, 3]);
        let result = two_sum_02(vec![-3,4,3,90], 0);
        assert_eq!(result, vec![0,2]);
    }

    #[test]
    fn test_two_sum_03() {
        let result = two_sum_03(vec![2,7,11,15],9);
        assert_eq!(result, vec![0, 1]);
        let result = two_sum_03(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
        let result = two_sum_03(vec![2,7,11,15],9);
        assert_eq!(result, vec![0, 1]);
        let result = two_sum_03(vec![0, 4, 3, 0], 0);
        assert_eq!(result, vec![0, 3]);
        let result = two_sum_03(vec![-3,4,3,90], 0);
        assert_eq!(result, vec![0,2]);
    }

}

