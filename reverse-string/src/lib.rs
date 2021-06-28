pub fn reverse(input: &str) -> String {
    let mut rev_str = String::new();
    for ch in input.chars().rev() {
        rev_str.push(ch);
    }
    rev_str
}
