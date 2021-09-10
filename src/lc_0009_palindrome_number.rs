// use crate::reverse_integer_7;

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x > 2_147_447_412 {
        // max palindrome int
        return false;
    } else if x < 10 {
        return true;
    } else if 0 == x % 10 {
        // looks like 210 or 10
        return false;
    } else {
        // let mut rst = 0 as u64;
        let mut vv = vec![];
        let mut t = x;
        loop {
            vv.append(&mut vec![t % 10]);
            t = t / 10;
            if 0 == t {
                break;
            }
        }
        let l = vv.len();
        for i in 0..l {
            if vv[i] != vv[l - 1 - i] {
                return false;
            }
        }
        return true;
    }

    // return false;
}

#[cfg(test)]
mod palindrome_number_test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, is_palindrome(123321), "123321 is palindrome");
    }
}
