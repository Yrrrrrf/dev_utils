# Dev Utils

[<img alt="github" src="https://img.shields.io/badge/github-Yrrrrrf%2Fdev__utils-58A6FF?style=for-the-badge&logo=github" height="24">](https://github.com/Yrrrrrf/dev_utils)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dev_utils.svg?style=for-the-badge&logo=rust" height="24">](https://crates.io/crates/dev_utils)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dev__utils-66c2a5?style=for-the-badge&labelColor=555555" height="24">](https://docs.rs/dev_utils)

`dev_utils` is a collection of utilities for use in development. Designed to be a comprehensive crate, containing a wide variety of tools for use in development. **Intended to be used as a dependency in other projects**, and as such, it is designed to be as modular as possible, allowing users to only include the features they need.

It is a manifestation of a crate where we do not need a deep dependencies tree to use a single feature. **This crate shouldn't be using any dependencies but for now, it uses the `log` crate for logging utilities**.

`Important Note`: This crate is currently in active development, and as such, it is not advisable for widespread utilization at this time. There are numerous features still in the process of implementation, which need to be thoroughly tested before they can be considered production-ready.

## Features
- [ ] `log`
    - [X] `rlog` - Log to stdout
    - [ ] `record` - Record logs to a file
    - [ ] `log` - Log struct for storing log data
- [ ] `conversion`
    - [X] `datetime` - UNIX timestamp, and date and time utilities
    - [ ] `base change` - Convert between bases (any base to any base)
- [ ] `codex` - Encode and decode data
    - [ ] `base64` - Encode and decode base64 data
    - [ ] `unicode` - Encode and decode unicode data
    - [ ] `gzip` - Encode and decode gzip data
- [ ] `console`
    - [ ] `readline` - Interactive readline (for use in a REPL (interactive shell))
    - [ ] `ansi` - ANSI escape codes for colors, styles, and cursor movement
        - [X] ANSI escape codes for colors
        - [ ] ANSI escape codes for styles, and cursor movement
        - [ ] Interactive readline
- [ ] `crypto`
    - [ ] `hash` - Hashing utilities (message digest)
    - [ ] `cipher` - Cipher utilities (encryption & decryption)
    - [ ] `key` - Key utilities (key generation)
- [ ] `files` - Easy file manipulation utilities
    - [X] `crud` - Create, read, update, and delete files
    - [ ] `list` - List files and directories in a directory
    - [ ] `copy` - Copy a file from one location to another
    - [ ] `move` - Move a file from one location to another
    - [ ] `rename` - Rename a file or directory

<!--
- [ ] `crypto`
    - [ ] `hash` - Hashing utilities
        - [ ] `md5` - MD5 hashing
        - [ ] `sha1` - SHA1 hashing
        - [ ] `sha2` - SHA2 hashing
        - [ ] `sha3` - SHA3 hashing
        - [ ] `blake2` - BLAKE2 hashing
        - [ ] `argon2` - Argon2 hashing
        - [ ] `bcrypt` - BCrypt hashing
        - [ ] `scrypt` - Scrypt hashing
    - [ ] `cipher` - Cipher utilities
        - [ ] `aes` - AES encryption
        - [ ] `des` - DES encryption
        - [ ] `rsa` - RSA encryption
        - [ ] `ecc` - ECC encryption
        - [ ] `otp` - One-time pad encryption
    - [ ] `key` - Key utilities
        - [ ] `aes` - AES key generation
        - [ ] `des` - DES key generation
        - [ ] `rsa` - RSA key generation
        - [ ] `ecc` - ECC key generation
        - [ ] `otp` - One-time pad key generation
-->

## Usage

To use this crate, add the following to your [`Cargo.toml`](Cargo.toml) file:

```toml
[dependencies]
dev_utils = "0.*"  # Add the latest version of this crate
log = "0.4.*"  # It also depends on the log crate, so add that too
```

## [License](LICENSE)

This project is licensed under the terms of the [MIT license](./LICENSE)
