# ShinyID

[![Crates.io](https://img.shields.io/crates/v/shinyid.svg)](https://crates.io/crates/shinyid)
[![Docs.rs](https://docs.rs/shinyid/badge.svg)](https://docs.rs/shinyid)

## About

ShinyID is a high-performance Rust package inspired by the Instagram shortcode system. It allows you to encode and decode unique identifiers (IDs) into a human-readable and URL-safe string format called 'shiny'. This package is designed for scenarios where speed and efficiency are crucial, making it ideal for applications that need to handle large volumes of encoded IDs.

## Table of Contents

- [ShinyID](#shinyid)
  - [About](#about)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Example](#example)
  - [License](#license)
  - [Author](#author)

## Features

- **Efficient Encoding :** ShinyID offers a highly efficient method for converting numeric IDs into shiny strings.
- **Lightning-Fast Decoding :** Decoding shiny strings into their original numeric IDs is optimized for speed and performance.
- **URL-Safe :** Shiny strings are designed to be URL-safe, making them suitable for web applications.
- **Inspired by Instagram :** The package draws inspiration from Instagram's shortcode system, ensuring a familiar and intuitive approach to representing IDs.

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
shinyid = "0.1.0"
```

## Example

Here's a simple example showcasing the use of ShinyID:

```rust
use shinyid::{to_id, to_shiny};
fn main() {
    let id = 18446744073709551615;
    let shiny = to_shiny(id);
    println!("Shiny representation of {} is {}", id, shiny);

    let shiny = "P__________";
    match to_id(shiny) {
        Ok(id) => println!("ID corresponding to {} is {}", shiny, id),
        Err(err) => eprintln!("Error: {}", err),
    }
}

```

## License

This package is distributed under the Apache License, Version 2.0. See the [LICENSE](https://github.com/itpey/shinyid-rs/blob/main/LICENSE) file for more details.

## Author

ShinyID was created by [itpey](https://github.com/itpey).
