
// s between 1 to 3999, I to MMMCMXCIX
pub fn roman_to_int(s: String) -> i32 {
    let vs = s.chars().collect::<Vec<char>>();

    let vl = vs.len();
    if 1 == vl {
        return c_to_i(vs[0]);
    }

    let mut v = 0;
    for i in 0 .. vl {
        let v1 = c_to_i(vs[i]);
        let v2 = if i != 0 && v1 > c_to_i(vs[i - 1]) { c_to_i(vs[i - 1]) } else { 0 };
        v = v + v1 - 2 * v2; // v2 added twice
    }

    v
}

fn c_to_i(c: char) -> i32 {
    match c {
        'M' => 1000,
        'D' => 500,
        'C' => 100,
        'L' => 50,
        'X' => 10,
        'V' => 5,
        'I' => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod roman_to_int_test {
    use super::*;

    #[test]
    fn test_roman_to_int_4() {
        assert_eq!(4, roman_to_int(String::from("IV")));
    }

    #[test]
    fn test_roman_to_int_17() {
        assert_eq!(17, roman_to_int(String::from("XVII")));
    }

    #[test]
    fn test_roman_to_int_1994() {
        assert_eq!(1994, roman_to_int(String::from("MCMXCIV")));
    }

    #[test]
    fn test_roman_to_int_1904() {
        assert_eq!(1904, roman_to_int(String::from("MCMIV")));
    }

}
