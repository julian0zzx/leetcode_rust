pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s;
    } else {
        let sl = s.len();

        let mut start = 0;
        let mut end = start + 1;
        let mut wide = end - start;

        for i in 0 .. sl {
            let mut j = sl as i32;
            while j >= i as i32 {
                let (f , w) = figure_palindrome_2(s.get(i .. j as usize).unwrap());
                if f && w > wide {
                    wide = w;
                    start = i;
                    end = j as usize;
                }
                j = j - 1;
            }
            if wide > sl / 2 { // nature character of palindrome
                break;
            }
        }

        return s.get(start .. end).unwrap().to_string();
    }
}

pub fn longest_palindrome_3(s: String) -> String {
    if s.len() == 1 {
        return s;
    } else {
        let ss = s.len();
        let vv = s.as_bytes();

        let mut start = 0;
        let mut end = start + 1;
        let mut wide = end - start;

        for i in 0 .. ss {
            for j in i .. ss {
                let (f , w) = figure_palindrome(&vv[i .. j+1]);
                if f && w > wide {
                    wide = w;
                    start = i;
                    end = j+1;
                }
            }
        }

        return s.get(start .. end).unwrap().to_string();
    }
}

pub fn longest_palindrome_2(s: String) -> String {
    if s.len() == 1 {
        return s;
    } else {
        let ss = s.len();
        let vv = s.as_bytes();

        let mut start = 0;
        let mut end = start + 1;
        let mut wide = end - start;

        for i in 0 .. ss {
            let mut flag = true; // try to figure out
            let s1 = i;
            let mut e1 = i + 2;
            if e1 > ss {
                e1 = ss;
            }
            
            while flag {
                let (f, w) = figure_palindrome(&vv[s1 .. e1]);
                flag = !f; // true -> break
                
                if f && w > wide { // update range
                    wide = w;
                    start = s1;
                    end = e1;
                }

                if e1 + 1 > ss { // it's not, with last char
                    break;
                } else {
                    e1 = e1 + 1;
                }
            }
        }

        return s.get(start .. end).unwrap().to_string();
    }
}

pub fn figure_palindrome(vv : &[u8]) -> (bool, usize) {
    if vv.len() == 1 {
        return (false, 1);
    }
    let size = vv.len();
    let mut flag = true;
    let rs = size / 2;
    for i in 0 .. rs {
        if vv[i] != vv[size - 1 - i] { // compare from head and tail
            flag = false;
            break;
        }
    }
    (flag, size)
}

pub fn figure_palindrome_2(vv : &str) -> (bool, usize) {
    if vv.len() == 1 {
        return (false, 1);
    }
    let size = vv.len();
    let mut flag = true;
    let rs = size / 2;
    for i in 0 .. rs {
        if vv[i .. i + 1] != vv[size - i - 1.. size - i] { // compare from head and tail
            flag = false;
            break;
        }
    }
    (flag, size)
}

#[cfg(test)]
mod longest_palindrome_test {
    use super::*;

    #[test]
    pub fn test_longest_palindrome_bab() { // or aba
        let input = "babad";
        let output = "bab";
        assert_eq!(output, longest_palindrome(String::from(input)));
    }

    #[test]
    pub fn test_longest_palindrome_bb() {
        let input = "cbbd";
        let output = "bb";
        assert_eq!(output, longest_palindrome(String::from(input)));
    }

    #[test]
    pub fn test_longest_palindrome_a1() {
        let input = "a";
        let output = "a";
        assert_eq!(output, longest_palindrome(String::from(input)));
    }

    #[test]
    pub fn test_longest_palindrome_a() {
        let input = "ac";
        let output = "a";
        assert_eq!(output, longest_palindrome(String::from(input)));
    }

    #[test]
    pub fn test_longest_palindrome_nn() {
        let input = "ann";
        let output = "nn";
        assert_eq!(output, longest_palindrome(String::from(input)));
    }

}
