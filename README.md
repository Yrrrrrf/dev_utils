<h1 align="center">
  <!-- Optional: Consider a simple, generic icon representing "tools" or "utilities" if you want. 
       Even a well-formatted text title is fine. Let's assume no icon for now, but you could add one.
  <img src="PATH_TO_YOUR_OPTIONAL_ICON.png" alt="Dev Utils Icon" width="128" height="128"> 
  -->
  <div align="center">Dev Utils</div>
</h1>

<div align="center">

[![GitHub](https://img.shields.io/badge/github-Yrrrrrf%2Fdev__utils-58A6FF?&logo=github)](https://github.com/Yrrrrrf/dev_utils)
[![Crates.io](https://img.shields.io/crates/v/dev_utils.svg?&logo=rust)](https://crates.io/crates/dev_utils)
[![Docs.rs](https://img.shields.io/badge/docs.rs-dev__utils-66c2a5?&labelColor=555555)](https://docs.rs/dev_utils)
[![Crates.io Downloads](https://img.shields.io/crates/d/dev_utils)](https://crates.io/crates/dev_utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

</div>

## Overview

**Dev Utils** is a comprehensive Rust workspace providing a collection of utility crates designed to streamline common development tasks and enhance productivity. Whether you're building CLIs, scripts, or larger applications, Dev Utils offers a robust toolkit for logging, file manipulation, data formatting, and more.

This project aims to provide well-tested, ergonomic, and practical tools, born out of real-world development needs and a desire to explore Rust's capabilities.

## Key Features

*   **Enhanced Logging (`dlog`):** A flexible logging system with multiple levels, colored output, and customizable formatting, distinct from but inspired by the `log` crate.
*   **Powerful File Operations (`file`):** Simplified CRUD operations, directory listing, recursive copying, and file finding capabilities.
*   **Terminal Text Styling (`format`):** Easy-to-use text coloring (RGB) and styling (bold, italic, etc.) for rich terminal output.
*   **Arbitrary Base Conversion (`base_change`):** Convert numbers between various bases (2-62), supporting integers and fractional parts.
*   **Custom DateTime Utilities (`datetime`):** Structs and methods for date/time representation, timestamp conversion, and parsing.
*   **Project Info Macro (`app_dt!`):** Quickly display your crate's `Cargo.toml` information (name, version, custom fields) in your application, great for CLIs.
*   **Procedural Macros (`dev_macros`):** (Work in Progress) A dedicated crate for custom procedural macros to reduce boilerplate and add powerful compile-time functionalities.
*   **Zero Boilerplate (Goal):** Aims to provide ready-to-use utilities that require minimal setup.

## Installation

Add `dev_utils` to your `Cargo.toml` dependencies:

```toml
[dependencies]
dev_utils = "0.1.1" # Replace with the latest version
```

<!-- Or if you want to install the CLI tool that uses it, if you build one -->

## Quick Start

Here's a glimpse of what you can do with `dev_utils`:

```rust
use dev_utils::app_dt;

fn main() {
    // * Display application information from Cargo.toml
    app_dt!(file!()); // This macro clears the screen and prints.

    app_dt!(file!(), "package" => ["license", "description"]); // Or select specific fields:
}
```

## Examples

Detailed examples showcasing various features of `dev_utils` can be found in the [`dev_utils/examples/`](./dev_utils/examples/) directory. Each example is designed to be run directly and demonstrates specific functionalities.

*   **[Main Tester](./dev_utils/examples/main_tester.rs)**: A general testbed for various features.
*   **[Base Change](./dev_utils/examples/base_change.rs)**: Demonstrates number base conversions.
*   **[Formatting](./dev_utils/examples/formatting.rs)**: Showcases text styling and coloring.
*   **[Dlog](./dev_utils/examples/dlog.rs)**: Illustrates the use of the custom logging module.
*   *(And others as you add them)*

To run an example (e.g., `main_tester` from within the `yrrrrrrf-dev_utils` root directory):
```sh
cargo run --package dev_utils --example main_tester  # from the root of the workspace
cargo run --example main_tester  # from within the dev_utils directory
```

## Contributing

We welcome contributions to the Dev Utils Project! If you'd like to contribute, please:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix (`git checkout -b feature/your-feature-name`).
3.  Make your changes and commit them with clear, descriptive messages.
4.  Push your changes to your fork (`git push origin feature/your-feature-name`).
5.  Create a pull request to the main repository.

Please ensure your code adheres to the existing style, includes appropriate tests, and is well-documented.

## Future Goals / Roadmap (Optional)

*   Finalize and integrate `dev_macros` for common boilerplate reduction (e.g., smart constructors).
*   Expand `file` utilities with more advanced features (e.g., watching file changes).
*   Introduce features for light-weight benchmarking.
*   Explore adding utilities for simple network requests or configuration management.
<!-- Add any other specific goals you have -->

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
