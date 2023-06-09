use csv::Reader;

fn main() {
    const IN: &'static str = r#"first_name;last_name;username
"Rob";"Pike";rob
# lines beginning with a # character are ignored
Ken;Thompson;ken
"Robert";"Griesemer";"gri"
"#;

    let mut r = Reader::new(IN.as_bytes());
    r.comma = ';';
    r.comment = Some('#');

    let got = r
        .read_all()
        .map(|v| format!("{v:?}"))
        .expect("read all failed");

    const EXPECT: &'static str = std::concat!(
        r#"[["first_name", "last_name", "username"], "#,
        r#"["Rob", "Pike", "rob"], "#,
        r#"["Ken", "Thompson", "ken"], "#,
        r#"["Robert", "Griesemer", "gri"]]"#
    );
    assert_eq!(EXPECT, got);
}
