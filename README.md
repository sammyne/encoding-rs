# encoding-rs

![build status](https://github.com/sammyne/encoding-rs/workflows/build/badge.svg)
[![docs badge](https://img.shields.io/badge/docs-0.5.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.65.0%2B-blue)

This repository tries to implement a Go-like encoding library in Rust.

## Overview

Supported encodings go as follow

> Click the encoding link would show the doc page of that encoding.

- [x] ascii85
- [ ] asn1
- [ ] base32
- [ ] base58
- [x] [base64][base64-doc]: implements base64 encoding as specified by [RFC 4648].
- [ ] [binary][binary-doc]
- [x] [csv][csv-doc]: reads and writes comma-separated values (CSV) files formatted as [RFC 4180].
- [x] [hex][hex-doc]: implements hex encoding.
- [ ] json: use [serde_json]
- [x] [pem][pem-doc]: implements the PEM data encoding as specified by [RFC 1421].
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

[base64-doc]: https://sammyne.github.io/encoding-rs/base64/
[binary-doc]: https://sammyne.github.io/encoding-rs/binary/
[csv-doc]: https://sammyne.github.io/encoding-rs/csv/
[hex-doc]: https://sammyne.github.io/encoding-rs/hex/
[pem-doc]: https://sammyne.github.io/encoding-rs/pem/
[serde_json]: https://crates.io/crates/serde_json
[RFC 1421]: https://rfc-editor.org/rfc/rfc1421.html
[RFC 4180]: https://rfc-editor.org/rfc/rfc4180.html
[RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html
