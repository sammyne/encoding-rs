use std::io::Write;

use hex::Dumper;

fn main() {
    let lines = vec![
        "Go is an open source programming language.",
        "\n",
        "We encourage all Go users to subscribe to golang-announce.",
    ];

    let mut out = vec![];
    let mut dumper = Dumper::new(&mut out);
    for line in lines {
        let _ = dumper.write(line.as_bytes());
    }
    let _ = dumper.flush();

    let got = unsafe { std::str::from_utf8_unchecked(out.as_slice()) };

    const EXPECT: &'static str = r#"00000000  47 6f 20 69 73 20 61 6e  20 6f 70 65 6e 20 73 6f  |Go is an open so|
00000010  75 72 63 65 20 70 72 6f  67 72 61 6d 6d 69 6e 67  |urce programming|
00000020  20 6c 61 6e 67 75 61 67  65 2e 0a 57 65 20 65 6e  | language..We en|
00000030  63 6f 75 72 61 67 65 20  61 6c 6c 20 47 6f 20 75  |courage all Go u|
00000040  73 65 72 73 20 74 6f 20  73 75 62 73 63 72 69 62  |sers to subscrib|
00000050  65 20 74 6f 20 67 6f 6c  61 6e 67 2d 61 6e 6e 6f  |e to golang-anno|
00000060  75 6e 63 65 2e                                    |unce.|
"#;

    assert_eq!(EXPECT, got);
}
