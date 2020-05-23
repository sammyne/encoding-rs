# encoding-rs

[![CircleCI](https://circleci.com/gh/sammyne/encoding-rs.svg?style=svg)](https://circleci.com/gh/sammyne/encoding-rs)
![rust](https://github.com/sammyne/encoding-rs/workflows/rust/badge.svg?branch=master)
[![docs badge](https://img.shields.io/badge/docs-0.2.0-blue)](https://sammyne.github.io/encoding-rs/encoding/)
![minimum rustc](https://img.shields.io/badge/rustc-1.43%2B-blue)

This repository tries to implement a Go-like encoding library in Rust.

## Overview 

Supported encodings go as follow 

- [ ] base58
- [ ] base64
- [x] binary
- [x] hex

## Head Ups
- Stable rust doesn't support benchmark well, so [criterion](https://crates.io/crates/criterion) is used for now.
