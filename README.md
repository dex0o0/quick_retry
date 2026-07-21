[![Crates.io](https://img.shields.io/crates/v/quick_retry.svg)](https://crates.io/crates/quick_retry)
[![Documentation](https://docs.rs/quick_retry/badge.svg)](https://docs.rs/quick_retry)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A lightweight, **zero-dependency** Rust library for retrying fallible operations with flexible delay and backoff strategies.

---

## Features

- **Zero External Dependencies:** Fast compile times and zero bloat.
- **Builder Pattern:** Clean, chainable API for configuring retry behaviors.
- **Exponential Backoff:** Easily configure delay scaling factors for resilient network/IO operations.
- **Zero-Cost Abstractions:** Lightweight implementation designed for maximum performance.
- **100% Safe & Tested:** Fully covered with unit tests and doc tests.

---

## Installation

Add `quick_retry` to your `Cargo.toml`:

```toml
[dependencies]
quick_retry = "0.1.0"
```

## Quick Start

### Basic Usage

Retry an operation up to 3 times with a default 100ms fixed delay:

```Rust
use quick_retry::Retry;
use std::time::Duration;

fn main() {
    let result = Retry::new()
        .max_attempts(3)
        .delay(Duration::from_millis(100))
        .run(|| {
            // Your fallible operation (I/O, network request, database query)
            fetch_data()
        });

    match result {
        Ok(data) => println!("Success: {}", data),
        Err(err) => eprintln!("Failed after retries: {}", err),
    }
}

fn fetch_data() -> Result<&'static str, &'static str> {
    // Simulating operation
    Ok("Data fetched successfully!")
}
```

## Advanced Usage: Exponential Backoff

increase delay exponential after each failed attempt (e.g 100ms --> 200ms --> 400ms):

```Rust
use quick_retry::Retry;
use std::time::Duration;

fn main() {
    let result = Retry::new()
        .max_attempts(5)
        .delay(Duration::from_millis(100))
        .exponential_backoff(2.0) // Doubles the delay each time
        .run(|| {
            connect_to_server()
        });

    if let Ok(connection) = result {
        println!("Connected: {:?}", connection);
    }
}

fn connect_to_server() -> Result<&'static str, &'static str> {
    Err("Connection timed out")
}
```

## API Reference

| Method                     | Dercription                                                  | Default                                     |
| -------------------------- | ------------------------------------------------------------ | ------------------------------------------- |
| `Retry::new()`             | Creates `Retry` build instance.                              | `max_attempts: 3`, `delay: 100ms` , `Fixed` |
| `.max_attempts(u32)`       | Sets the maximum number of attempts before failing.          | `3`                                         |
| `.delay(Duration)`         | Sets the initial delay duration between                      | `100ms`                                     |
| `exponential_backoff(f64)` | Enables exponential backoff with th given multiplier factor. | Disabled(`Fixed`)                           |
| `.run(F)`                  | Executes the operation closure `FnMut() -> Result<T,E>.`     | N/A                                         |

## License

This project Licensed under the MIT License-see the LICENSE file for details.

## Support & Feadback

if you find this crate useful, please consider given it a star on [GitHub](https://github/dex0o0/quick_retry)
