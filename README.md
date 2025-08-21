# librust

Librust is a low-level Rust library that provides an errno handler and syscall utilities.

## Overview

This repository contains core components for interacting with system-level functionality in Rust, focusing on error handling and syscall abstractions. Librust is designed for developers building low-level or systems software in Rust who need direct access to OS features with robust error management.

## Features

- **Errno Handler:** Simplifies error handling from system calls.
- **Syscall Utilities:** Easy-to-use wrappers for system calls.
- **Rust Language:** Written entirely in Rust for safety and performance.

## Getting Started

### Usage

Clone the repository:

```bash
git clone https://github.com/bkoshik/librust.git
cd librust
```

Add to your project’s `Cargo.toml`:

```toml
[dependencies]
librust = { git = "https://github.com/bkoshik/librust.git" }
```

Import in your Rust code:

```rust
use librust::error;      // Errno handler
use librust::syscall;    // Syscall
use librust::unix;       // Syscall wrappers
```

## Contributing

Issues, suggestions, and pull requests are welcome!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a pull request

## License

Licensed under the [Apache License 2.0](LICENSE).

## Author

Maintained by [bkoshik](https://github.com/bkoshik)