# SysMonk

[![made-with-rust][rust-logo]][rust-src-page]

[![crates.io][crates-logo]][crate]

[![build][gh-logo]][build]
[![none-shall-pass][nsp-logo]][nsp]

#### Summary
[`SysMonk`][repo] is a system monitoring tool that provides a simple and easy-to-use interface to monitor system resources. It is designed to be lightweight and fast.

### Installation

```shell
cargo add SysMonk
```

### Usage
```rust
use sysmonk;

#[actix_rt::main]
async fn main() {
    match sysmonk::start().await {
        Ok(_) => {
            println!("SysMonk session has ended")
        }
        Err(err) => {
            eprintln!("Error starting SysMonk: {}", err)
        }
    }
}
```

<details>
<summary><strong>Download OS specific Executable</strong></summary>

###### macOS (x86_64)
```shell
curl -o SysMonk-Darwin-x86_64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/SysMonk/releases/latest/download/SysMonk-Darwin-x86_64.tar.gz"
```

###### macOS (arm64)
```shell
curl -o SysMonk-Darwin-arm64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/SysMonk/releases/latest/download/SysMonk-Darwin-arm64.tar.gz"
```

###### Linux (x86_64)
```shell
curl -o SysMonk-Linux-x86_64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/SysMonk/releases/latest/download/SysMonk-Linux-x86_64.tar.gz"
```

###### Windows (x86_64)
```shell
curl -o SysMonk-Windows-x86_64.zip -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/SysMonk/releases/latest/download/SysMonk-Windows-x86_64.zip"
```
</details>

#### Environment Variables

**Mandatory**
- **username**: Username for the API server.
- **password**: Password for the API server.

**Optional**
- **debug**: Boolean flag to enable debug level logging. Defaults to `false`
- **utc_logging**: Boolean flag to set timezone to UTC in the output logs. Defaults to `true`
- **host**: IP address to host the server. Defaults to `127.0.0.1` / `localhost`
- **port**: Port number to host the application. Defaults to `8000`
- **session_duration**: Time _(in seconds)_ each authenticated session should last. Defaults to `3600`
- **workers**: Number of workers to spin up for the server. Defaults to the number of physical cores.
- **max_connections**: Maximum number of concurrent connections per worker. Defaults to `3`
- **websites**: Vector of websites (_supports regex_) to add to CORS configuration. _Required only if tunneled via CDN_

## Crate
[https://crates.io/crates/SysMonk][crate]

### Cargo Docs - Official Runbook
[https://docs.rs/SysMonk/latest/sysmonk/][docs]

**Generator**
```shell
cargo doc --document-private-items --no-deps
```

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix
```

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][license]

[repo]: https://github.com/thevickypedia/SysMonk
[license]: https://github.com/thevickypedia/SysMonk/blob/main/LICENSE
[build]: https://github.com/thevickypedia/SysMonk/actions/workflows/rust.yml
[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust
[gh-logo]: https://github.com/thevickypedia/SysMonk/actions/workflows/rust.yml/badge.svg
[nsp-logo]: https://github.com/thevickypedia/SysMonk/actions/workflows/none.yml/badge.svg
[nsp]: https://github.com/thevickypedia/SysMonk/actions/workflows/none.yml
[crate]: https://crates.io/crates/SysMonk
[gh-checks]: https://github.com/thevickypedia/SysMonk/actions/workflows/rust.yml
[crates-logo]: https://img.shields.io/crates/v/SysMonk.svg
[docs]: https://docs.rs/SysMonk/latest/sysmonk/
