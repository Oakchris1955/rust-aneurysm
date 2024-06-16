# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.1 - 2024-06-16

### Added

- This CHANGELOG to make tracking of changes to the repository easier
- An `-e --echo` flag for echoing anything inputted to stdin back to stdout (useful for debugging)
- Commit tags for [crates.io] releases

### Changed

- Users can now override the logging level set by the program by using the `RUST_LOG` environment variable, Furthermore, debug messages will now be printed if the program was compiled with debug assertations.
- README is now more verbose and informative

## 0.1.0 - 2024-06-14

First release on [crates.io]

### Added

- Brainf**k interpreter that has been thoroughly tested to make sure that bugs are non-existent
- Various logging levels, which can be changed by using the `-v --verbose` flag
- Custom modular type to perform modular arithmetic on the data pointer. This essentialy makes the data pointer wraparound the cell memory when it goes out-of-bounds.
- Barebones README

[crates.io]: https://crates.io
