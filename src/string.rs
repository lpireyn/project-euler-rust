pub fn reverse(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    s.chars().rev().collect()
}

pub fn is_palindrome(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    reverse(s) == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("abc"), "cba");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(""));
        assert!(!is_palindrome("abcb"));
        assert!(is_palindrome("abcba"));
    }
}
