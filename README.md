# `flips.rs` [![Star me](https://img.shields.io/github/stars/althonos/flips.rs.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/flips.rs/stargazers)

*Rust bindings to [Flips](https://github.com/Alcaro/Flips), the Floating IPS patcher.*

[![TravisCI](https://img.shields.io/travis/com/althonos/flips.rs/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/althonos/flips.rs/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/flips.rs/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/flips.rs)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/gpl-3.0/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/flips.rs)
[![Crate](https://img.shields.io/crates/v/flips.svg?maxAge=600&style=flat-square)](https://crates.io/crates/flips)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/flips)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/flips.rs/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/althonos/flips.rs.svg?style=flat-square&maxAge=600)](https://github.com/althonos/flips.rs/issues)


## 🗺️ Overview

[Flips](https://github.com/Alcaro/Flips) is a popular patcher for the IPS, BPS
and UPS formats, typically used to patch ROMs of video game cartridges. It is
known to create the smallest BPS and IPS files among all widely used patchers.
This library provides a safe API to create and apply patches to arbitrary
sources.

| Format | Apply | Create | Metadata | Study |
| ------ | ----- | ------ | -------- | ----- |
| UPS    | ✔️     |        |          |       |
| IPS    | ✔️     | ✔️      |          | ✔️     |
| BPS    | ✔️     | ✔️      | ✔️        |       |

## 🔌 Usage

Load a ROM and a patch from two files, apply the patch to the ROM, and then
write it back to a file:

```rust
extern crate flips;

// get the input ROM and patch
let patch = std::fs::read("FE_LonelyMirror_v3_3.ups").unwrap();
let rom = std::fs::read("Fire Emblem 8.rom").unwrap();

// apply the patch and write the output
let output = flips::UpsPatch::new(patch).apply(rom)
  .expect("could not apply patch");
std::fs::write("FE_LonelyMirror.rom", output).unwrap();
```

Check the [online documentation](https://docs.rs/flips) for more examples about
how to use this library.

## 📝 Features

### 📦 `no_std` support

`no_std` support for this crate can be opted-in by disabling the **`std`**
feature. It will disable support of [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
and [`Vec<u8>`](https://doc.rust-lang.org/std/vec/struct.Vec.html). It will
also disable dynamic dispatch of hardware-accelerated CRC32 implementation.

### 🧩 CRC32

Flips is patched to use the [`crc32fast`](https://crates.io/crates/crc32fast)
crate instead of the naive algorithm it used, which greatly improves performances
when creating or applying BPS and UPS patches, since both of this formats will
compute the checksum for their inputs and outputs every time.

## 📋 Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/flips.rs/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.

## 📜 License

This library is provided under the
[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/),
since Flips itself is GPLv3 software.
