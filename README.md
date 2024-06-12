# Yet Another Brainf**k interpreter

An interpreter for `.bf` files (or any file that is written for the Brainf\*\*k *programming* language) that is written in the Rust programming language and is designed with efficiency in mind

## Installing / Compiling

### SOON

## Usage

```text
A Brainf**k interpreter written in Rust with minimal dependencies

Usage: aneurysm [OPTIONS] [filename]

Arguments:
  [filename]  Brainf**k file to execute [default: main.bf]

Options:
  -m, --mem <memory>  The memory size in bytes/cells to allocate for the program [default: 30000]
  -v, --verbose       Enable verbose logging
  -h, --help          Print help
  -V, --version       Print version
```

## TODO

- [ ] Add generics for the `WrappingUInt` struct, rename it to `Modular` and move it to its own submodule
- [ ] Separate crate for `modular` submodule
- [ ] Debugger (preferable some kind of remote server that can be used with existing debuggers)
- [ ] Compiler?
