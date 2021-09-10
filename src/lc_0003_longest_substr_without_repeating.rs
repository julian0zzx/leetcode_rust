
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut is = 0;
    let mut es = 0;
    let mut mx = 0;
    let slen = s.len();
    if 1 >= slen {
        return slen as i32;
    }

    loop {
        if is >= slen - 1 || (es >= slen && is + mx >= slen) {
            break;
        }
        loop {
            if is >= slen - 1 || (es >= slen && is + mx >= slen) {
                break;
            }

            let ss = s.get(is .. es).unwrap();
            let c = s.get(es .. es + 1).unwrap();
            if !ss.contains(c) {
                es = es + 1;
                let mx2 = ss.len() + 1;
                if mx2 > mx {
                    mx = mx2;
                }
            } else {
                is = is + 1;
                es = is + 1;
            }
        }
    }

    mx as i32
}

#[cfg(test)]
mod length_of_longest_substring_test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_3() {
        assert_eq!(3, length_of_longest_substring(String::from("abcabcbb")));
    }

    #[test]
    fn test_length_of_longest_substring_1() {
        assert_eq!(1, length_of_longest_substring(String::from("bbbbbbb")));
    }

    #[test]
    fn test_length_of_longest_substring_32() {
        assert_eq!(3, length_of_longest_substring(String::from("pwwkew")));
    }

    #[test]
    fn test_length_of_longest_substring_33() {
        assert_eq!(3, length_of_longest_substring(String::from("dvdf")));
    }

    #[test]
    fn test_length_of_longest_substring_0() {
        assert_eq!(0, length_of_longest_substring(String::from("")));
    }

    #[test]
    fn test_length_of_longest_substring_11() {
        assert_eq!(1, length_of_longest_substring(String::from("a")));
    }

    #[test]
    fn test_length_of_longest_substring_2() {
        assert_eq!(2, length_of_longest_substring(String::from("au")));
    } 
}
