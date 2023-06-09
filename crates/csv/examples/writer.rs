use csv::Writer;

fn main() {
    let records = vec![
        vec!["first_name", "last_name", "username"],
        vec!["Rob", "Pike", "rob"],
        vec!["Ken", "Thompson", "ken"],
        vec!["Robert", "Griesemer", "gri"],
    ];

    let mut out = vec![];
    {
        let mut w = Writer::new(&mut out);

        for v in records {
            w.write(v).expect("writing record to csv");
        }

        // Write any buffered data to the underlying writer.
        w.flush().unwrap();

        if let Some(err) = w.error() {
            panic!("unexpected error: {err:?}");
        }
    }

    const EXPECT: &'static str = r#"first_name,last_name,username
Rob,Pike,rob
Ken,Thompson,ken
Robert,Griesemer,gri
"#;

    let got = unsafe { std::str::from_utf8_unchecked(out.as_slice()) };

    assert_eq!(EXPECT, got);
}
