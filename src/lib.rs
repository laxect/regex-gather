pub use error::Error;
use error::Result;
use regex::{escape, Regex};
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
pub use utils::Predicate;

mod error;
mod utils;

/// just scan from first to last.
fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let first = strs.first().unwrap();
    let mut len = first.graphemes(true).count();
    for str in &strs[1..] {
        len = cmp::min(
            len,
            str.graphemes(true)
                .zip(first.graphemes(true))
                .take_while(|&(a, b)| a == b)
                .count(),
        );
    }
    first.graphemes(true).take(len).collect()
}

/// just scan from first to last too.
/// just reverse.
fn longest_common_subfix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let first = strs.first().unwrap();
    let mut len = first.graphemes(true).count();
    let count = len;
    for str in &strs[1..] {
        len = cmp::min(
            len,
            str.graphemes(true)
                .rev()
                .zip(first.graphemes(true).rev())
                // reverse
                .take_while(|&(a, b)| a == b)
                .count(),
        );
    }
    // len always small than first.len()
    first.graphemes(true).skip(count - len).collect()
}

/// gather pattern from input filename.
/// need at least 2 for gather regex.
pub fn gather_regex(input: &[&str], diff_mark: &str, ext_num: bool) -> Result<Regex> {
    // need at least 2 for gather regex
    if input.len() < 2 {
        return Err(Error::NoEnoughInput);
    }
    let mut prefix = longest_common_prefix(input);
    let mut subfix = longest_common_subfix(input);
    if ext_num {
        prefix = prefix.trim_end_matches(char::is_numeric).to_string();
        subfix = subfix.trim_start_matches(char::is_numeric).to_string();
    }
    let pattern = format!("{}{}{}", escape(&prefix), diff_mark, escape(&subfix));

    Ok(Regex::new(&pattern).expect("should always success"))
}

/// gather pattern from input filename.
/// need at least 2 for gather pattern.
/// use a escape function.
pub fn gather_pattern_with_escape<F: Fn(&str) -> String>(
    input: &[&str],
    diff_mark: &str,
    ext_num: bool,
    escape: F,
) -> Result<String> {
    // need at least 2 for gather regex
    if input.len() < 2 {
        return Err(Error::NoEnoughInput);
    }
    let mut prefix = longest_common_prefix(input);
    let mut subfix = longest_common_subfix(input);
    if ext_num {
        prefix = prefix.trim_end_matches(char::is_numeric).to_string();
        subfix = subfix.trim_start_matches(char::is_numeric).to_string();
    }
    let pattern = format!("{}{}{}", escape(&prefix), diff_mark, escape(&subfix));
    Ok(pattern)
}

/// gather pattern from input filename without escape.
/// need at least 2 for gather pattern.
pub fn gather_pattern(input: &[&str], diff_mark: &str, ext_num: bool) -> Result<String> {
    // need at least 2 for gather regex
    if input.len() < 2 {
        return Err(Error::NoEnoughInput);
    }
    let mut prefix = longest_common_prefix(input);
    let mut subfix = longest_common_subfix(input);
    if ext_num {
        prefix = prefix.trim_end_matches(char::is_numeric).to_string();
        subfix = subfix.trim_start_matches(char::is_numeric).to_string();
    }
    let pattern = format!("{}{}{}", &prefix, diff_mark, &subfix);
    Ok(pattern)
}

/// try guess pattern from only one input.
/// simplely replace any number with diff mark.
/// will raise a error when no number in input.
pub fn guess_pattern_with_escape<F: Fn(&str) -> String>(
    input: &str,
    diff_mark: &str,
    escape: F,
) -> Result<Vec<Predicate>> {
    // not numeric
    let numeric = Regex::new("\\d+").unwrap();
    if !numeric.is_match(input) {
        return Err(Error::NoNumFound);
    }
    let offs: Vec<&str> = numeric.split(input).collect();
    let nums: Vec<&str> = input
        .split(|ch| !char::is_numeric(ch))
        .filter(|s| !s.is_empty())
        .collect();
    let mut res = Vec::new();
    for i in 0..nums.len() {
        // there should be i in nums and  i+1 in offs
        // should have one
        let mut pattern = offs[0].to_owned();
        let mut left = pattern.clone();
        let mut right = String::new();
        for j in 0..nums.len() {
            match i.cmp(&j) {
                cmp::Ordering::Less => {
                    right.push_str(offs[j + 1]);
                    right.push_str(nums[j]);
                    pattern.push_str(nums[j]);
                }
                cmp::Ordering::Equal => {
                    right.push_str(offs[j + 1]);
                    pattern.push_str(diff_mark);
                }
                cmp::Ordering::Greater => {
                    left.push_str(nums[j]);
                    left.push_str(offs[j + 1]);
                    pattern.push_str(nums[j]);
                }
            }
            pattern.push_str(&escape(offs[j + 1]));
        }
        let predicate = Predicate::new(pattern, left, right, nums[i].to_owned());
        res.push(predicate);
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_common_prefix_test() {
        let inputs = vec!["aaab", "aaacb", "aaeb", "aaeeb"];
        let res = longest_common_prefix(&inputs).len();
        assert_eq!(res, 2);
    }

    #[test]
    fn longest_common_subfix_test() {
        let inputs = vec!["aaab", "aaacb", "aaeb", "aaeeb"];

        let res = longest_common_subfix(&inputs).len();
        assert_eq!(res, 1);
    }

    #[test]
    fn longest_common_prefix_test_cjk() {
        let inputs = vec!["你好", "你不好", "你好", "你你你好"];
        let res = longest_common_prefix(&inputs);
        assert_eq!(res, "你");
    }

    #[test]
    fn longest_common_subfix_test_cjk() {
        let inputs = vec!["你好", "你不好", "你好", "你你你好"];
        let res = longest_common_subfix(&inputs);
        assert_eq!(res, "好");
    }

    #[test]
    fn gather_from_at_least_2_test() {
        let mark = "(?P<ep>.*)";
        let inputs = vec![
            "ひぐらしのなく頃に 業 01.mp4",
            "ひぐらしのなく頃に 業 02.mp4",
            "ひぐらしのなく頃に 業 03.mp4",
        ];
        let res = gather_regex(&inputs, mark, true).unwrap();
        assert_eq!(res.as_str(), "ひぐらしのなく頃に 業 (?P<ep>.*)\\.mp4");
    }

    #[test]
    fn template_test() {
        let mark = "{ep}";
        let inputs = vec![
            "ひぐらしのなく頃に 業 01.mp4",
            "ひぐらしのなく頃に 業 02.mp4",
            "ひぐらしのなく頃に 業 03.mp4",
        ];
        let res = gather_pattern_with_escape(&inputs, mark, true, |s| s.to_string()).unwrap();
        assert_eq!(res.as_str(), "ひぐらしのなく頃に 業 {ep}.mp4");
    }
}
