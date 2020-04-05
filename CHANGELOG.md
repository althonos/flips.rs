# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
[Unreleased]: https://github.com/althonos/flips.rs/compare/v0.2.0...HEAD

## [v0.2.0] - 2020-04-05
[v0.2.0]: https://github.com/althonos/flips.rs/compare/v0.1.0...v0.2.0
### Added
- Support for `no_std` compilation by disabling `std` feature.
- Delta BPS patches with more memory support.
### Changed
- Make `flips-sys` use `crc32fast` instead of builtin Flips implementation
  to compute checksums of patches and ROMs.

## [v0.1.0] - 2020-03-22
Initial release.
[v0.1.0]: https://github.com/althonos/flips.rs/compare/3bd54de...v0.1.0
