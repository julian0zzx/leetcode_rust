pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let fstr = &strs[0]; // first String MUST contain the longest prefix

    for i in 1..fstr.len() + 1 {
        let lpre = fstr.split_at(i).0;
        for s in &strs {
            if !s.starts_with(lpre) {
                if 1 == lpre.len() {
                    return String::from("");
                }
                return String::from(lpre.split_at(lpre.len() - 1).0);
            }
        }
        if lpre.len() == fstr.len() {
            return String::from(lpre);
        }
    }

    String::from("")
}

#[cfg(test)]
mod longest_common_prefix_test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            String::from("abc"),
            String::from("abcd"),
            String::from("abced"),
            String::from("abf"),
        ];
        assert_eq!(String::from("ab"), longest_common_prefix(strs));
    }

    #[test]
    fn test_longest_common_prefix_none() {
        let strs = vec![
            String::from("abc"),
            String::from("cabcd"),
            String::from("abced"),
            String::from("abf"),
        ];
        assert_eq!(String::from(""), longest_common_prefix(strs));
    }
}
