use std::io::Write;

#[test]
fn line_breaker() {
    let test_vector = vec![
        Test::new("", ""),
        Test::new("a", "a\n"),
        Test::new("ab", "ab\n"),
        Test::new(
            SIXTY_FOUR_CHAR_STRING,
            SIXTY_FOUR_CHAR_STRING.to_string() + "\n",
        ),
        Test::new(
            SIXTY_FOUR_CHAR_STRING.to_owned() + "X",
            SIXTY_FOUR_CHAR_STRING.to_owned() + "\nX\n",
        ),
        Test::new(
            SIXTY_FOUR_CHAR_STRING.to_owned() + SIXTY_FOUR_CHAR_STRING,
            SIXTY_FOUR_CHAR_STRING.to_string() + "\n" + SIXTY_FOUR_CHAR_STRING + "\n",
        ),
    ];

    for (i, test) in test_vector.iter().skip(3).enumerate() {
        let mut buf = vec![];

        let mut breaker = super::new(&mut buf);
        breaker
            .write_all(test.input.as_bytes())
            .expect(&format!("#{i} write"));
        breaker.flush().expect(&format!("#{i} flush"));

        std::mem::drop(breaker);

        let got = unsafe { String::from_utf8_unchecked(buf) };
        assert_eq!(test.out, got, "#{i}");
    }
}

const SIXTY_FOUR_CHAR_STRING: &'static str =
    "0123456789012345678901234567890123456789012345678901234567890123";

struct Test {
    input: String,
    out: String,
}

impl Test {
    fn new<S, T>(input: S, out: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            input: input.into(),
            out: out.into(),
        }
    }
}
