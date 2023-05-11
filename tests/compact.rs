#[cfg(test)]
mod tests {
    #[test]
    pub fn compare() {
        use case_insensitive_string::CaseInsensitiveString;
        let case_insensitive = CaseInsensitiveString::from("iDk");

        // both of the strings are a match!
        assert_eq!(case_insensitive, CaseInsensitiveString::from("IDK"))
    }
}
