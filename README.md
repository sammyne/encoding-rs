# encoding-rs

![build status](https://github.com/sammyne/encoding-rs/workflows/build/badge.svg)
[![docs badge](https://img.shields.io/badge/docs-0.4.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.65.0%2B-blue)

This repository tries to implement a Go-like encoding library in Rust.

## Overview

Supported encodings go as follow

> Click the encoding link would show the doc page of that encoding.

- [x] ascii85
- [ ] asn1
- [ ] base32
- [ ] base58
- [ ] base64
- [ ] [binary][binary-doc]
- [x] [csv][csv-doc]: reads and writes comma-separated values (CSV) files formatted as [RFC 4180].
- [x] [hex][hex-doc]: implements hex encoding.
- [ ] json: use [serde_json]
- [ ] pem
- [ ] xml

## Quickstart

### tests

```bash
cargo test
```

### benchmark

```bash
cargo bench
```

## Head Ups

- Stable rust doesn't support benchmark well, so [criterion](https://crates.io/crates/criterion) is used for now.
- docs is released at the `gh-pages` branch for the `main` branch only

[binary-doc]: https://sammyne.github.io/encoding-rs/binary/
[csv-doc]: https://sammyne.github.io/encoding-rs/csv/
[hex-doc]: https://sammyne.github.io/encoding-rs/hex/
[serde_json]: https://crates.io/crates/serde_json
[RFC 4180]: https://rfc-editor.org/rfc/rfc4180.html
