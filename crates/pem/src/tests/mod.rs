#[test]
fn get_line() {
    let test_vector = vec![
        GetLineTest::new("abc", "abc", ""),
        GetLineTest::new("abc\r", "abc\r", ""),
        GetLineTest::new("abc\n", "abc", ""),
        GetLineTest::new("abc\r\n", "abc", ""),
        GetLineTest::new("abc\nd", "abc", "d"),
        GetLineTest::new("abc\r\nd", "abc", "d"),
        GetLineTest::new("\nabc", "", "abc"),
        GetLineTest::new("\r\nabc", "", "abc"),
        GetLineTest::new("abc\t \nd", "abc", "d"),
        GetLineTest::new("\t abc\nd", "\t abc", "d"),
        GetLineTest::new("abc\n\t d", "abc", "\t d"),
        GetLineTest::new("abc\nd\t ", "abc", "d\t "),
    ];

    for (i, test) in test_vector.iter().enumerate() {
        let (x, y) = super::get_line(test.input.as_bytes());

        let x = unsafe { std::str::from_utf8_unchecked(x) };
        let y = unsafe { std::str::from_utf8_unchecked(y) };

        assert_eq!(
            test.out1,
            x,
            "#{i} {}: bad out1",
            test.input.escape_unicode().to_string()
        );
        assert_eq!(
            test.out2,
            y,
            "#{i} {}: bad out2",
            test.input.escape_unicode().to_string()
        );
    }
}

struct GetLineTest {
    input: &'static str,
    out1: &'static str,
    out2: &'static str,
}

impl GetLineTest {
    fn new(input: &'static str, out1: &'static str, out2: &'static str) -> Self {
        Self { input, out1, out2 }
    }
}
