# Dev Utils

[![GitHub](https://img.shields.io/badge/github-Yrrrrrf%2Fdev__utils-58A6FF?style=for-the-badge&logo=github)](https://github.com/Yrrrrrf/dev_utils)
[![Crates.io](https://img.shields.io/crates/v/dev_utils.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/dev_utils)
[![Docs.rs](https://img.shields.io/badge/docs.rs-dev__utils-66c2a5?style=for-the-badge&labelColor=555555)](https://docs.rs/dev_utils)

Welcome to the Dev Utils Project! This repository contains a collection of Rust crates and utilities designed to streamline development processes and provide helpful tools for various programming tasks.


## Project Structure

The crate is organized using Rust workspaces, allowing for modular development and easy management of multiple related crates. The main crate is located in the [`dev_utils`](./dev_utils/) directory, with additional crates and utilities located in subdirectories.

```bash
dev_utils/
├── Cargo.toml  # workspace config
│   # Main crate
├── dev_utils/
│   ├── Cargo.toml
│   ├── src/  # main source files
│   │   ├── lib.rs  # crate entry point
│   │   └── ...
│   ├── examples/  # Example applications
│   └── README.md  # dev_utils crate README
│   # Custom macros (not yet implemented)
├── dev_macros/
│   ├── Cargo.toml
│   └── src/lib.rs
└── README.md   # this file
```

## Getting Started

Add the following to your `Cargo.toml`:
```toml
[dependencies]
dev_utils = "0.*"  # add the latest version
```

## Examples

The [`examples`](./dev_utils/examples/) directory contains several sample applications showcasing the usage of Dev Utils features.

## Contributing

We welcome contributions to the Dev Utils Project! If you'd like to contribute, please:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them with clear, descriptive messages
4. Push your changes to your fork
5. Create a pull request to the main repository

Please ensure your code adheres to the existing style and includes appropriate tests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to all contributors who have helped shape the Dev Utils Project.
- The Rust community for continuous inspiration and support.
