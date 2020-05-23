# encoding-rs

![master](https://github.com/sammyne/encoding-rs/workflows/build/badge.svg?branch=master)
[![docs badge](https://img.shields.io/badge/docs-0.2.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.43%2B-blue)


![dev](https://github.com/sammyne/encoding-rs/workflows/build-dev/badge.svg?branch=dev)

This repository tries to implement a Go-like encoding library in Rust.

## Overview 

Supported encodings go as follow 

- [ ] base58
- [ ] base64
- [x] binary
- [x] hex

## Head Ups
- Stable rust doesn't support benchmark well, so [criterion](https://crates.io/crates/criterion) is used for now.
- docs is released at the `gh-pages` branch for `master` branch only