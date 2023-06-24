pub fn cut<'a>(s: &'a [u8], sep: &[u8]) -> Option<(&'a [u8], &'a [u8])> {
    let idx = s
        .windows(sep.len())
        .enumerate()
        .find(|&(_, w)| w == sep)
        .map(|(i, _)| i)?;

    let before = &s[..idx];
    let after = &s[(idx + sep.len())..];

    Some((before, after))
}

pub fn index<'a>(s: &'a [u8], sep: &[u8]) -> Option<usize> {
    s.windows(sep.len())
        .enumerate()
        .find(|&(_, w)| w == sep)
        .map(|(i, _)| i)
}

/// ref: slice::trim_ascii_end
pub fn trim_right<'a>(s: &'a [u8], cutset: &str) -> &'a [u8] {
    let mut bytes = s;
    let cutset = cutset.as_bytes();
    // Note: A pattern matching based approach (instead of indexing) allows
    // making the function const.
    while let [rest @ .., last] = bytes {
        if cutset.contains(last) {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}

pub const fn trim_space(s: &[u8]) -> &[u8] {
    let mut out = s;
    while let [first, tail @ ..] = out {
        if first.is_ascii_whitespace() {
            out = tail;
        } else {
            break;
        }
    }

    while let [start @ .., last] = out {
        if last.is_ascii_whitespace() {
            out = start;
        } else {
            break;
        }
    }

    out
}
