
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    return nums.len() as i32;
}

pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut nn = vec![];
    for n in nums {
        if val != *n {
            nn.push(n);
        }
    }
    println!("{:#?}", nn);
    return nn.len() as i32;
}

#[cfg(test)]
mod remove_element_tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let nums = &mut vec![1, 2, 3, 4, 5, 5];
        let val = 5;
        assert_eq!(4, remove_element2(nums, val));
    }

    #[test]
    fn test_remove_element2() {
        let nums = &mut vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(2, remove_element2(nums, val));
    }

}

