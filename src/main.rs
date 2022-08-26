
mod lc_0002_add_two_numbers;
mod lc_0014_longest_common_prefix;
mod lc_0009_palindrome_number;
mod lc_0007_reverse_integer;
mod lc_0633_sum_of_square_numbers;
mod lc_0001_two_sum;
mod lc_0013_roman_to_integer;
mod lc_0012_integer_to_roman;
mod lc_0004_median_two_sorted_arrays;
mod lc_0003_longest_substr_without_repeating;
mod lc_0021_merge_two_sorted_lists;
mod lc_0008_string_to_integer_atoi;
mod lc_0005_longest_palindrome;
mod lc_0006_zigzag_conversion;
mod lc_0010_regular_expres;
mod lc_0020_valid_parentheses;
mod lc_0026_remove_duplicates;
mod lc_0027_remove_element;
mod lc_0028_impl_strstr;

use log::{debug, error, info, trace, warn};

fn main() {
    // println!("{:?}", lc_0020_valid_parentheses::is_valid("([)]".to_string()));
    // println!("{}", lc_0028_impl_strstr::str_str2("abc".to_string(), "c".to_string()));
    info!("{:?}", lc_0020_valid_parentheses::is_valid("([)]".to_string()).to_string());
    info!("{}", lc_0028_impl_strstr::str_str2("abc".to_string(), "c".to_string()));
}

fn convert(s: String, n: i32) -> String {
    // 0,1,..,n-1,n-2,n-3,..,1     (0 .. n) is the same as (0 .. =n-1)
    let it = (0..=n-1).into_iter().chain((1..=n-2).rev().into_iter());
    // let it = (0..n).into_iter().chain((1..n-1).rev().into_iter());
    let mut vr: Vec<Vec<char>> = vec![vec![]; n as usize];
    s.chars().zip(it.cycle()).for_each(|(c, i)| {print!("i-{},c-{},", i, c);vr[i as usize].push(c);});
    println!("{:?}", vr);
    vr.into_iter().flatten().collect()
}
