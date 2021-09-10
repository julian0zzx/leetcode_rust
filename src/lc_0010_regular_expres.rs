pub fn is_match(s: String, p: String) -> bool {

    false
}

#[cfg(test)]
mod regular_expres_test {
    use super::*;

    #[test]
    pub fn test_is_match_a() {
        assert_eq!(false, is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    pub fn test_is_match_a3() {
        assert_eq!(true, is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    pub fn test_is_match_ab() {
        assert_eq!(true, is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    pub fn test_is_match_aab() {
        assert_eq!(true, is_match("aab".to_string(), "c*a*b".to_string()));
    }

    #[test]
    pub fn test_is_match_ms() {
        assert_eq!(false, is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
    }

}
