# encoding-rs

![main](https://github.com/sammyne/encoding-rs/workflows/build/badge.svg?branch=main)
[![docs badge](https://img.shields.io/badge/docs-0.2.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.54%2B-blue)

![dev](https://github.com/sammyne/encoding-rs/workflows/build-dev/badge.svg?branch=dev)

This repository tries to implement a Go-like encoding library in Rust.

## Overview

Supported encodings go as follow

| scheme  | implementation          | document                               | comment |
| ------- | ----------------------- | -------------------------------------- | ------- |
| ascii85 |                         |
| asn1    |                         |
| base32  |                         |
| base58  |                         |
| base64  | :ballot_box_with_check: |                                        |         |
| binary  | :ballot_box_with_check: | [:ballot_box_with_check:][binary-docs] |
| csv     |                         |
| hex     | :ballot_box_with_check: |
| json    | N/A                     | use [serde_json]                       |
| pem     |                         |
| xml     |                         |

## Head Ups

- Stable rust doesn't support benchmark well, so [criterion](https://crates.io/crates/criterion) is used for now.
- docs is released at the `gh-pages` branch for the `master` branch only

[binary-docs]: https://sammyne.github.io/encoding-rs/encoding/binary/index.html
[serde_json]: https://crates.io/crates/serde_json
