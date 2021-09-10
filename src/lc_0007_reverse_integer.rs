pub fn reverse(x: i32) -> i32 {
    let a = x.abs();

    let mut rst = 0 as u64;
    let mut t = a as u64;
    while t >= 10 {
        rst = rst * 10 + t % 10;
        t = t / 10;
    }
    rst = rst * 10 + t;
    if rst > std::i32::MAX as u64 {
        return 0;
    }

    let rr = rst as i32;

    if x < 0 {
        return 0 - rr;
    }

    rr
}

#[cfg(test)]
mod reverse_integer_test {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(-321, reverse(-1230));
    }

    #[test]
    fn test_reverse_10() {
        assert_eq!(1, reverse(10));
    }

    #[test]
    fn test_reverse_max() {
        assert_eq!(0, reverse(1_534_236_469));
    }
}
