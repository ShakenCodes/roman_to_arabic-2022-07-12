#[cfg(test)]
mod tests {
    use roman_to_arabic::roman_to_arabic;
    #[test]
    fn given_empty_string_roman_to_arabic_returns_0() {
        assert_eq!(roman_to_arabic(""), 0);
    }
}
