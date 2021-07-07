pub use error::Error;
use error::Result;
use regex::{escape, Regex};
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

mod error;

/// just scan from first to last
pub fn longest_common_prefix<'s>(strs: &'s Vec<String>) -> String {
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

/// just scan from first to last too
/// just reverse
pub fn longest_common_subfix<'s>(strs: &'s Vec<String>) -> String {
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

pub fn gather_from_at_least_2(input: &Vec<String>, diff_mark: &str) -> Result<Regex> {
    // need at least 2 for gather regex
    if input.len() < 1 {
        return Err(Error::NoEnoughInput);
    }
    let prefix = longest_common_prefix(&input);
    let subfix = longest_common_subfix(&input);
    let pattern = format!(
        "{}(?P<{}>.*){}",
        escape(&prefix),
        diff_mark,
        escape(&subfix)
    );

    Ok(Regex::new(&pattern).expect("should always success"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_common_prefix_test() {
        let inputs = vec!["aaab", "aaacb", "aaeb", "aaeeb"];
        let inputs = inputs.into_iter().map(ToString::to_string).collect();
        let res = longest_common_prefix(&inputs).len();
        assert_eq!(res, 2);
    }

    #[test]
    fn longest_common_subfix_test() {
        let inputs = vec!["aaab", "aaacb", "aaeb", "aaeeb"];
        let inputs = inputs.into_iter().map(ToString::to_string).collect();
        let res = longest_common_subfix(&inputs).len();
        assert_eq!(res, 1);
    }

    #[test]
    fn longest_common_prefix_test_cjk() {
        let inputs = vec!["你好", "你不好", "你好", "你你你好"];
        let inputs = inputs.into_iter().map(ToString::to_string).collect();
        let res = longest_common_prefix(&inputs);
        assert_eq!(res, "你");
    }

    #[test]
    fn longest_common_subfix_test_cjk() {
        let inputs = vec!["你好", "你不好", "你好", "你你你好"];
        let inputs = inputs.into_iter().map(ToString::to_string).collect();
        let res = longest_common_subfix(&inputs);
        assert_eq!(res, "好");
    }

    #[test]
    fn gather_from_at_least_2_test() {
        let inputs = vec![
            "ひぐらしのなく頃に 業 1.mp4",
            "ひぐらしのなく頃に 業 2.mp4",
            "ひぐらしのなく頃に 業 3.mp4",
        ];
        let inputs = inputs.into_iter().map(ToString::to_string).collect();
        let res = gather_from_at_least_2(&inputs, "sp").unwrap();
        assert_eq!(res.as_str(), "ひぐらしのなく頃に 業 (?P<sp>.*)\\.mp4");
    }
}
