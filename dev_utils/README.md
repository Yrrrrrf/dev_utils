# Dev Utils

[<img alt="github" src="https://img.shields.io/badge/github-Yrrrrrf%2Fdev__utils-58A6FF?style=for-the-badge&logo=github" height="24">](https://github.com/Yrrrrrf/dev_utils)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dev_utils.svg?style=for-the-badge&logo=rust" height="24">](https://crates.io/crates/dev_utils)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dev__utils-66c2a5?style=for-the-badge&labelColor=555555" height="24">](https://docs.rs/dev_utils)

`dev_utils` is a collection of utilities for use in development. Designed to be a comprehensive crate, containing a wide variety of tools for use in development. **Intended to be used as a dependency in other projects**, and as such, it is designed to be as modular as possible, allowing users to only include the features they need.

## Features
- [x] `dlog` - dev log instance different from the `log` crate but same macros
- [x] `datetime` - UNIX timestamp, and date and time utilities
- [x] `base_change` - Convert between bases (any base to any base)
    - [ ] fix *some* bugs (when using decimals) 
- [x] `formatting` - Styling traits for formatting data (ANSI colors, bold, italic, underline, etc.)
- [X] `file` - Create, read, update, and delete files
    - [ ] other file manipulation utilities (list, copy, move, rename)

<!--
- [ ] `readline` - Interactive readline (for use in a REPL (interactive shell))
- [ ] `serde` - Serialize and deserialize data
- [ ] `dcrypto` - Dev crypto utilities (hashing, encryption, decryption, key generation)
- [ ] `codex` - Encode and decode data (base64, unicode, gzip)
- [ ] `crypto`
    - [ ] `cipher` - Cipher utilities
    - [ ] `key` - Key utilities
-->

## Usage

To use this crate, add the following to your [`Cargo.toml`](Cargo.toml) file:

```toml
[dependencies]
dev_utils = "0.15"
```

## [License](LICENSE)

This project is licensed under the terms of the [MIT license](./LICENSE)
