pub fn convert(s: String, num_rows: i32) -> String {
    let sl = s.len();
    if sl == 1 || sl == num_rows as usize || 1 == num_rows {
        return s;
    }

    let utsize = 2 * num_rows - 2; // unit size
    let mut vsize = sl as i32 / utsize;
    vsize = vsize * (num_rows - 1); // 2-d vec vertical size
    if sl as i32 % utsize > num_rows {
        vsize += 1 + (utsize - num_rows);
    } else {
        vsize += 1;
    }

    let mut zigzag = vec![vec!['-'; num_rows as usize]];
    for i in 0 .. vsize - 1 {
        zigzag.append(&mut vec![vec!['-'; num_rows as usize]]);
    }

    let mut ss : String = s.chars().rev().collect(); // it's cool for UTF8!
    let mut vi = 0;
    while vi <= vsize {
        for vhi in 0 .. num_rows { // first array in unit
            let so = ss.pop();
            if so.is_some() {
                zigzag[vi as usize][vhi as usize] = so.unwrap();
            }
        }

        let mut vhvi = num_rows - 2;
        while  vhvi > 0 {
            vi += 1;
            let so = ss.pop();
            if so.is_some() {
                zigzag[vi as usize][vhvi as usize] = so.unwrap();
            }
            vhvi -= 1;
        }
        
        vi += 1; 
    }

    // to push out
    let mut sss = "".to_string();
    for n in 0 .. num_rows {
        for v in 0 .. vsize {
            let c = zigzag[v as usize][n as usize];
            if '-' != c {
                sss.push(c);
            }
        }
    }

    sss
}

pub fn convert_m(s: String, n: i32) -> String {
    let it = (0..n).into_iter().chain((1..=n-2).rev().into_iter());
    println!("{:?}", it);
    let mut vr: Vec<Vec<char>> = vec![vec![]; n as usize];
    s.chars().zip(it.cycle()).for_each(|(c, i)| vr[i as usize].push(c));
    vr.into_iter().flatten().collect()
}

#[cfg(test)]
pub mod zigzag_conversion_test {
    use super::*;

    #[test]
    fn test_convert_a() {
        assert_eq!("a".to_string(), convert("a".to_string(), 1));
    }

    #[test]
    fn test_convert() {
        assert_eq!("PINALSIGYAHRPI".to_string(), convert("PAYPALISHIRING".to_string(), 4));
    }

}
