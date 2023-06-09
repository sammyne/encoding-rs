use csv::Writer;

fn main() {
    let records = vec![
        vec!["first_name", "last_name", "username"],
        vec!["Rob", "Pike", "rob"],
        vec!["Ken", "Thompson", "ken"],
        vec!["Robert", "Griesemer", "gri"],
    ];

    let mut out = vec![];

    let mut w = Writer::new(&mut out);

    w.write_all(records.as_ref())
        .expect("write out all records");

    if let Some(err) = w.error() {
        panic!("unexpected error: {err:?}");
    }
    std::mem::drop(w);

    const EXPECT: &'static str = r#"first_name,last_name,username
Rob,Pike,rob
Ken,Thompson,ken
Robert,Griesemer,gri
"#;

    let got = unsafe { std::str::from_utf8_unchecked(out.as_slice()) };

    assert_eq!(EXPECT, got);
}
