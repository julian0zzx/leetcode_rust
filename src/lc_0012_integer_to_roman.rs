
// 1 <= num <= 3999
pub fn int_to_roman(num: i32) -> String {
    let mut str = String::from("");
    let mut ori = num;
    let mut t10 = 1;
    loop {
        if t10 >= 10000 {
            break;
        }

        let l = ori % 10;

        if ori < 10 && 1 == t10 {
            str = format!("{}{}", str, i_to_r(ori));
        } else {
            str = format!("{}{}", i_to_r(l * t10), str);
        }

        t10 = t10 * 10;
        ori = ori / 10;

        if ori == 0 {
            break;
        }
    }
    
    str
}

fn i_to_r<'a >(n : i32) -> &'a str {
    match n {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        20 => "XX",
        30 => "XXX",
        40 => "XL",
        50 => "L",
        60 => "LX",
        70 => "LXX",
        80 => "LXXX",
        90 => "XC",
        100 => "C",
        200 => "CC",
        300 => "CCC",
        400 => "CD",
        500 => "D",
        600 => "DC",
        700 => "DCC",
        800 => "DCCC",
        900 => "CM",
        1000 => "M",
        2000 => "MM",
        3000 => "MMM",
        _ => "",
    }
}

#[cfg(test)]
mod int_to_roman_test {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!("XIV", int_to_roman(14));
    }
}
