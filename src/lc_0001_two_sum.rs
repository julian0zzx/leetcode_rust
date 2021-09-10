pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let vs = nums.len();
    for n in 0..vs {
        for m in n + 1..vs {
            if nums[n] + nums[m] == target {
                return vec![n as i32, m as i32];
            }
        }
    }
    vec![0, 0]
}

#[cfg(test)]
mod two_sum_tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let v = vec![3, 4, 5];
        let t = vec![1, 2];
        assert_eq!(t, two_sum(v, 9));
    }
}
