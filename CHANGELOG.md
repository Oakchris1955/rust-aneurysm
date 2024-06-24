# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

Debugger incoming

### Added

- Brainf\*\*k debugger (named `lobotomy`) to make it easier to write and debug Brainf\*\*k programs.
  It functions as a shell (powered by [`shellfish`](https://crates.io/crates/shellfish)), with commands written for this very purpose (with more coming soon).
  All the argument parsing is done using `clap`.
  It also stores log files in a local directory [according to](https://crates.io/crates/directories):
  - the XDG base directory and the XDG user directory specifications on Linux
  - the Known Folder API on Windows
  - the Standard Directories guidelines on macOS
- A new `reset()` method for the `Interpreter` struct, which does exactly what you think
- A new `reset()` method for the `Modular` struct, which, as above, does exactly what you think (only works if the inner type implements `Default`)

### Changed

- Split project into different libraries and binaries
- Log levels of some messages
- **IMPORTANT:** unmatched loop brackets will now be considered a syntax error (<https://brainfuck.org/brainfuck.html>)
- Some functions of the `Interpreter` struct, namely `new` and `new_from_path` will now return a specialized `Result` type, `InterpreterResult`

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

- Brainf\*\*k interpreter that has been thoroughly tested to make sure that bugs are non-existent
- Various logging levels, which can be changed by using the `-v --verbose` flag
- Custom modular type to perform modular arithmetic on the data pointer. This essentialy makes the data pointer wraparound the cell memory when it goes out-of-bounds.
- Barebones README

[crates.io]: https://crates.io
