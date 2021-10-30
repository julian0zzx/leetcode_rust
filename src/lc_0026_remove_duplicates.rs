
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if 0 == nums.len() {
        return 0;
    }

    let mut n = nums[0];
    let mut i = 1;
    loop {
        if i >= nums.len() {
            break;
        }
        let nn = nums[i];
        if n == nn {
            nums.remove(i);
        } else {
            n = nn;
            i += 1;
        }
    }

    return nums.len() as i32;
}

#[cfg(test)]
mod remove_duplicates_test {
    use super::*;

    #[test]
    pub fn test_remove_duplicates3() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates(&mut nums));
    }

    #[test]
    pub fn test_remove_duplicates10() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates(&mut nums));
    }

}
