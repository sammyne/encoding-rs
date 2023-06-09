# encoding-rs

![build status](https://github.com/sammyne/encoding-rs/workflows/build/badge.svg)
[![docs badge](https://img.shields.io/badge/docs-0.3.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.65.0%2B-blue)

This repository tries to implement a Go-like encoding library in Rust.

## Overview

Supported encodings go as follow

| scheme  | implementation          | document                               | comment |
| ------- | ----------------------- | -------------------------------------- | ------- |
| ascii85 | :ballot_box_with_check: |
| asn1    |                         |
| base32  |                         |
| base58  |                         |
| base64  | :ballot_box_with_check: |                                        |         |
| binary  | :ballot_box_with_check: | [:ballot_box_with_check:][binary-docs] |
| csv     | :ballot_box_with_check: |
| hex     | :ballot_box_with_check: |
| json    | N/A                     | use [serde_json]                       |
| pem     |                         |
| xml     |                         |

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

[binary-docs]: https://sammyne.github.io/encoding-rs/encoding/binary/index.html
[serde_json]: https://crates.io/crates/serde_json
