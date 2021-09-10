
pub fn my_atoi(s: String) -> i32 {
    // remove space
    let mut s2 = s.trim().chars().collect::<Vec<char>>();
    if s2.len() == 0 {
        return 0;
    }

    let c0 = s2[0];
    if c0 != '-' && c0 != '+' && c0 != '0' && c0 != '1' && c0 != '2' && c0 != '3' && c0 != '4' 
        && c0 != '5' && c0 != '6' && c0 != '7' && c0 != '8' && c0 != '9' {
            return 0;
    }
    
    s2 = s.trim().split_whitespace().next().unwrap().chars().collect::<Vec<char>>();

    let is_neg = if c0 == '-' { true } else { false };
    if '-' == s2[0] || '+' ==s2[0] { // remove '-'
        s2 = s2[1 .. ].to_vec();
    } else {
        s2 = s2;
    }
    let mut r : i64 = 0;
    for c in s2 {
        if is_digital(c) {
            r = r * 10 + iatoi(c) as i64;
            if (r as i64 ) > std::i32::MAX as i64 {
                if is_neg {
                    return std::i32::MIN;
                } else {
                    return std::i32::MAX;
                }
            }
        } else {
            break;
        }
    }

    if is_neg {
        if ( (0 - r) as i64) < std::i32::MIN as i64 {
            return std::i32::MIN;
        } else {
            return (0 - r) as i32;
        }
    } else {
        if (r as i64 ) > std::i32::MAX as i64 {
            return std::i32::MAX;
        } else {
            return r as i32;
        }
    }
}

fn is_digital(c : char) -> bool {
    match c {
        '0' => true,
        '1' => true,
        '2' => true,
        '3' => true,
        '4' => true,
        '5' => true,
        '6' => true,
        '7' => true,
        '8' => true,
        '9' => true,
        _ => false
    }
}

fn iatoi(c : char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0
    }
}

#[cfg(test)]
mod str_to_integer_test {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(0, my_atoi(String::from("0")));
    }

    #[test]
    fn test_my_atoi_42() {
        assert_eq!(42, my_atoi(String::from("42")));
    }

    #[test]
    fn test_my_atoi_n42() {
        assert_eq!(-42, my_atoi(String::from("   -42")));
    }

    #[test]
    fn test_my_atoi_4193() {
        assert_eq!(4193, my_atoi(String::from("4193 with words")));
    }

    #[test]
    fn test_my_atoi_0() {
        assert_eq!(0, my_atoi(String::from("words and 987")));
    }

    #[test]
    fn test_my_atoi_min() {
        assert_eq!(-2147483648, my_atoi(String::from("-91283472332")));
    }

    #[test]
    fn test_my_atoi_max() {
        assert_eq!(2147483647, my_atoi(String::from("31283472332")));
    }

    #[test]
    fn test_my_atoi_p1() {
        assert_eq!(1, my_atoi(String::from("+1")));
    }

    #[test]
    fn test_my_atoi_ps() {
        assert_eq!(0, my_atoi(String::from("  ")));
    }

    #[test]
    fn test_my_atoi_pp() {
        assert_eq!(3, my_atoi(String::from(" 3.1415926 ")));
    }

    #[test]
    fn test_my_atoi_ppd() { // not a 
        assert_eq!(-12, my_atoi(String::from("  -0012a42")));
    }
}
