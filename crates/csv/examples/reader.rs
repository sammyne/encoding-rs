use csv::{ReadError, Reader};

fn main() {
    const IN: &'static str = r#"first_name,last_name,username
"Rob","Pike",rob
Ken,Thompson,ken
"Robert","Griesemer","gri"
"#;

    let mut r = Reader::new(IN.as_bytes());

    let mut got = String::new();
    loop {
        match r.read() {
            Err(err) if std::matches!(err.err, ReadError::Eof) => break,
            Err(err) => panic!("{err}"),
            Ok(v) => got += &format!("{v:?}\n"),
        }
    }

    const EXPECT: &'static str = r#"["first_name", "last_name", "username"]
["Rob", "Pike", "rob"]
["Ken", "Thompson", "ken"]
["Robert", "Griesemer", "gri"]
"#;

    assert_eq!(EXPECT, got, "unexpected output");
}
