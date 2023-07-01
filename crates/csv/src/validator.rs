pub fn valid_delimiter(c: char) -> bool {
    !matches!(c, '\0' | '"' | '\r' | '\n' | char::REPLACEMENT_CHARACTER)
}
