Ntplib
------------

[![Build Status](https://travis-ci.org/JeffBelgum/ntp.svg?branch=master)](https://travis-ci.org/JeffBelgum/ntp)
[![Documentation](https://docs.rs/ntp/badge.svg)](https://docs.rs/ntp)
[![Crates.io](https://img.shields.io/crates/v/ntp.svg?maxAge=2592000)](https://crates.io/crates/ntp)
[![License](https://img.shields.io/crates/l/ntp.svg)](https://github.com/JeffBelgum/ntp#license)

An ntp packet parsing library written in Rust.


Usage
-----

Add this to your `Cargo.toml`:

```ini
[dependencies]
ntp = "0.5"
```

and this to your crate root:

```rust
extern crate ntp;
```

Todo
----

- [ ] no-std
- [ ] io independent parsing
- [ ] async support
- [ ] setting clocks
- [ ] ntp server functionality


Contributions
-------------

Pull Requests and Issues welcome!

License
-------

`ntp` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
