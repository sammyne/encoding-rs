# encoding-rs

![master](https://github.com/sammyne/encoding-rs/workflows/build/badge.svg?branch=master)
[![docs badge](https://img.shields.io/badge/docs-0.2.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.43%2B-blue)


![dev](https://github.com/sammyne/encoding-rs/workflows/build-dev/badge.svg?branch=dev)

This repository tries to implement a Go-like encoding library in Rust.

## Overview 

Supported encodings go as follow 

| scheme | implementation          | document                                                                                    | comment |
| ------ | ----------------------- | ------------------------------------------------------------------------------------------- | ------- |
| base58 |                         |
| base64 | doing                   |                                                                                             |         |
| binary | :ballot_box_with_check: | [:ballot_box_with_check:](https://sammyne.github.io/encoding-rs/encoding/binary/index.html) |
| hex    | :ballot_box_with_check: |

## Head Ups
- Stable rust doesn't support benchmark well, so [criterion](https://crates.io/crates/criterion) is used for now.
- docs is released at the `gh-pages` branch for the `master` branch only