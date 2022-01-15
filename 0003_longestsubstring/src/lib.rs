use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {
    if !s.is_ascii() {
        panic!("Unexpected characters");
    }
    // window over the mini of (length_of_all_symbols, length_of_string)
    //  I'm sure there is a super long string in the test set 5 * 104
    //  Consider the window and then inserting into a hashset, check the len against the window, if the len and the window are the same, the size should be correct.
    let capacity = 126 - 32 + 1; // The ascii set size. Ascii values: Dec: 32 (space) .. 126 (~), size: u8
    let mut set: HashSet<&u8> = HashSet::with_capacity(capacity);
    let mut window = usize::min(s.len(), capacity);
    let sbytes = s.as_bytes();
    while window > 0 {
        let found_longest = sbytes.windows(window).any(|b| {
            set = b.iter().collect();
            set.len() == window
        });
        if found_longest {
            return window as i32;
        }
        window -= 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(
            3,
            length_of_longest_substring(String::from_str("abcabcbb").unwrap())
        );
    }

    #[test]
    fn test_02() {
        assert_eq!(
            1,
            length_of_longest_substring(String::from_str("bbbbb").unwrap())
        );
    }

    #[test]
    fn test_03() {
        assert_eq!(
            3,
            length_of_longest_substring(String::from_str("pwwkew").unwrap())
        );
    }
}
