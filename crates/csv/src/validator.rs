pub fn valid_delimiter(c: char) -> bool {
    match c {
        '\0' | '"' | '\r' | '\n' | '�' => false,
        _ => true,
    }
}
