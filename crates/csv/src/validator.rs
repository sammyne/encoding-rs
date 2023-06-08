pub fn valid_delimiter(c: char) -> bool {
    match c {
        '\0' | '"' | '\r' | '\n' | char::REPLACEMENT_CHARACTER => false,
        _ => true,
    }
}
