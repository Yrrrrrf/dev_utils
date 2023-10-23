# Dev Utils

[<img alt="github" src="https://img.shields.io/badge/github-Yrrrrrf%2Fdev__utils-58A6FF?style=for-the-badge&logo=github" height="24">](https://github.com/Yrrrrrf/dev_utils)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dev_utils.svg?style=for-the-badge&logo=rust" height="24">](https://crates.io/crates/dev_utils)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dev__utils-66c2a5?style=for-the-badge&labelColor=555555" height="24">](https://docs.rs/dev_utils)

Welcome to our comprehensive development utility crate. Please note that while we offer these tools to the community, they are primarily designed for internal use, and we are actively working to enhance and expand our offerings.

`Important Note`: This crate is currently in active development, and as such, it is not advisable for widespread utilization at this time. There are numerous features still in the process of implementation, which need to be thoroughly tested before they can be considered production-ready.

## Features
- [X] `log` - Logging utilities
- [X] `datetime` - Date and time utilities
- [X] `files` - File manipulation utilities (CRUD)
- [X] `terminal` - Terminal utilities
    - [X] ANSI escape codes for colors
    - [ ] ANSI escape codes for styles, and cursor movement
    - [ ] Interactive readline
- [ ] `siu` - Simple Interactive Utilities (for CLI applications)
- [ ] `util` - General utilities
    - [ ] base change
    - [ ] encyption
    - [ ] hashing (idk becaus it's *already in stdlib*)
    - [ ] encoding
    - [ ] compression (at a later date)

## Usage

To use this crate, add the following to your `Cargo.toml` file:

```toml
[dependencies]
dev_utils = "0.*"  # Add the latest version of this crate
log = "*"  # It also depends on the log crate, so add that too
```

## [License](LICENSE)

This project is licensed under the terms of the [MIT license](./LICENSE)
