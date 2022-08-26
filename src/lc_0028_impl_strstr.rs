
// haystack and needle are in lowercase

pub fn str_str(haystack: String, needle: String) -> i32 {
    if 0 == needle.len() {
        return 0;
    }
    if haystack.len() < needle.len() {
        return -1;
    }
    if haystack == needle {
        return 0;
    }

    let hl = haystack.len();
    let nl = needle.len();

    let mut idx = 0;
    while idx < hl {
        if idx + nl > hl {
            return -1;
        }
        let hs = haystack.get(idx .. idx + nl).unwrap();
        if needle == hs {
            return idx as i32;
        } else {
            idx += 1;
        }
    }

    return -1;
}

pub fn str_str2(haystack: String, needle: String) -> i32 {
    if 0 == needle.len() {
        return 0;
    }
    if haystack.len() < needle.len() {
        return -1;
    }

    let hc = haystack.as_bytes(); // into_bytes
    let nc = needle.as_bytes();

    let hl = haystack.len();
    let nl = needle.len();
    let mut idx = 0;
    while idx < hl {
        if hc[idx] == nc[0] {
            for i in 0 .. nl {
                if hc[idx+i] != nc[i] {
                    idx += i;
                    break;
                } else {
                    return idx as i32;
                }
            }
        }
        idx += 1;
    }

    return -1;
}

#[cfg(test)]
mod test_impl_strstr {
    use super::*;

    #[test]
    fn test_impl_strstr() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn test_impl_strstr2() {
        assert_eq!(str_str("abc".to_string(), "c".to_string()), 2);
    }

    #[test]
    fn test_impl_strstr3() {
        assert_eq!(str_str("mississippi".to_string(), "issi".to_string()), 1);
    }

    #[test]
    fn test_impl_strstr4() {
        assert_eq!(str_str2("mississippi".to_string(), "issi".to_string()), 1);
    }

    #[test]
    fn test_impl_strstr5() {
        assert_eq!(str_str2("abc".to_string(), "c".to_string()), 2);
    }

    #[test]
    fn test_impl_strstr6() {
        assert_eq!(str_str2("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn test_impl_strstr7() {
        assert_eq!(str_str2("aaa".to_string(), "aaaa".to_string()), -1);
    }

    #[test]
    fn test_impl_strstr8() {
        assert_eq!(str_str2("aaa".to_string(), "aaa".to_string()), 0);
    }
}
