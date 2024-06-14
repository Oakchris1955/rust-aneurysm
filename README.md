# Yet Another Brainf**k interpreter

An interpreter for `.bf` files (or any file that is written for the Brainf\*\*k *programming* language) that is written in the Rust programming language and is designed with efficiency in mind

## Installing / Compiling

If you have Cargo installed, simply run `cargo install aneurysm`. If you have already installed this package and want to update to the latest version, run `cargo install --force aneurysm`

## Usage

```text
A Brainf**k interpreter written in Rust with minimal dependencies

Usage: aneurysm [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  Brainf**k file to execute [default: main.bf]

Options:
  -m, --mem <memory>  The memory size in bytes/cells to allocate for the program [default: 30000]
  -v, --verbose       Enable verbose logging
  -e, --echo          Whether or not to echo characters written to stdin
  -h, --help          Print help
  -V, --version       Print version
```

### Logging

Verbose logging will be printed to stderr when the `-v --verbose` flag is set. Anything with a level of `INFO` or above will be printed, or `DEBUG` is the program is run with debug assertations on. If the flag isn't set, the default level will be `WARN`. Please note that you can set the logging level at runtime using the `RUST_LOG` environment variable, which will take precedence over the above

## TODO

- [x] Add generics for the `WrappingUInt` struct, rename it to `Modular` and move it to its own submodule
- [ ] Separate crate for `modular` submodule
- [ ] Debugger (preferable some kind of remote server that can be used with existing debuggers)
- [ ] Compiler?
