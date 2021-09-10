// double pointer, 145 = 1^2 + 12^2
pub fn judge_square_sum(c: i32) -> bool {
    let csqrt = (c as f64).sqrt() as i32;

    let mut l = 0;
    let mut r = csqrt;

    while l <= r {
        let s = l * l + r * r;
        if s == c {
            println!("{}, {}, {}", c, l, r);
            return true;
        } else if s < c {
            l = l + 1;
        } else {
            r = r - 1;
        }
    }

    false
}

// stackoverflow, 145 = 8^2 + 9^2
pub fn judge_square_sum2(c: i32) -> bool {
    let csqrt = (c as f64).sqrt() as i32;
    for a1 in 0..csqrt {
        let a12 = a1 * a1;
        if a12 > c {
            return false;
        } else if a12 == c {
            return true; // c = a1^2 + 0^2
        } else {
            for a2 in a1..csqrt {
                if a12 + a2 * a2 == c {
                    println!("{}, {}, {}", c, a1, a2);
                    return true;
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod sum_of_square_numbers_tests {
    use super::*;

    #[test]
    fn test_judge_square_sum() {
        assert_eq!(true, judge_square_sum(145));
    }

    #[test]
    fn test_judge_square_sum2() {
        assert_eq!(true, judge_square_sum2(145));
    }
}
