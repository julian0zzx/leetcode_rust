pub fn is_valid(s: String) -> bool {
    let sl = s.len();
    if sl % 2 == 1 { // length is odd
        return false;
    }
    if 2 == sl {
        return is_pair(s.get(0 .. 1).unwrap(), s.get(1 .. 2).unwrap());
    } else {
        let mut ss = s;
        for i in 0 .. sl {
            if i + 2 > sl {
                return false;
            }
            let l = ss.get(i .. i + 1).unwrap();
            let r = ss.get(i + 1 .. i + 2).unwrap();
            if is_pair(l, r) {
                ss.drain(i .. i + 2);
                println!("{}", ss);
                return is_valid(ss);
            }
        }
    }

    return false;
}

// break test_is_match_f8 
pub fn is_valid_2(s: String) -> bool {
    let sl = s.len();
    if sl % 2 == 1 { // length is odd
        return false;
    }

    if 2 == sl {
        return is_pair(s.get(0 .. 1).unwrap(), s.get(1 .. 2).unwrap());
    } else {
        let ml = s.get(sl/2 -1 .. sl/2).unwrap();
        let mr = s.get(sl/2 .. sl/2 + 1).unwrap();
        if is_pair(ml, mr) {
            let l = s.get(sl/2 - 2 .. sl/2 - 1).unwrap();
            if is_right(l) { // >|[
                if is_valid(s.get(0 .. sl/2 - 1).unwrap().to_string()) {
                    return is_valid(s.get(sl/2 + 1 .. sl).unwrap().to_string());
                } else {
                    return false;
                }
            } else { // <|}
                let mut step = 1;
                while step <= sl / 2 - 1 {
                    let ll = s.get(sl/2 - step - 1 .. sl/2 - step).unwrap();
                    let rr = s.get(sl/2 + step .. sl/2 + step + 1).unwrap();
                    if !is_pair(ll, rr) {
                        return false;
                    }
                    step += 1;
                }
                return true;
            }
        } else {
            return false;
        } 
    }
}

fn is_pair(l : &str, r : &str) -> bool {
    "(".eq(l) && ")".eq(r) || "[".eq(l) && "]".eq(r) || "{".eq(l) && "}".eq(r)
}

fn is_left(l : &str) -> bool {
    "(".eq(l) || "[".eq(l) || "{".eq(l)
}

fn is_right(r : &str) -> bool {
    ")".eq(r) || "]".eq(r) || "}".eq(r)
}

#[cfg(test)]
mod valid_parentheses_test {
    use super::*;

    #[test]
    pub fn test_is_match_1() {
        assert_eq!(true, is_valid("()".to_string()));
    }

    #[test]
    pub fn test_is_match_2() {
        assert_eq!(true, is_valid("()[]{}".to_string()));
    }

    #[test]
    pub fn test_is_match_f2() {
        assert_eq!(false, is_valid("(([]{}".to_string()));
    }

    #[test]
    pub fn test_is_match_a() {
        assert_eq!(false, is_valid("(]".to_string()));
    }

    #[test]
    pub fn test_is_match_b() {
        assert_eq!(false, is_valid("([)]".to_string()));
    }

    #[test]
    pub fn test_is_match_c() {
        assert_eq!(true, is_valid("{[()]}".to_string()));
    }

    #[test]
    pub fn test_is_match_fc() {
        assert_eq!(false, is_valid("{]()]}".to_string()));
    }

    #[test]
    pub fn test_is_match_f4() {
        assert_eq!(false, is_valid("(()(".to_string()));
    }

    #[test]
    pub fn test_is_match_f8() {
        assert_eq!(true, is_valid("(([]){})".to_string()));
    }
}
