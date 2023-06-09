fn main() {
    let content = b"Go is an open source programming language.";
    let got = hex::dump(content);

    const EXPECT: &'static str = r#"00000000  47 6f 20 69 73 20 61 6e  20 6f 70 65 6e 20 73 6f  |Go is an open so|
00000010  75 72 63 65 20 70 72 6f  67 72 61 6d 6d 69 6e 67  |urce programming|
00000020  20 6c 61 6e 67 75 61 67  65 2e                    | language.|
"#;

    assert_eq!(EXPECT, got);
}
