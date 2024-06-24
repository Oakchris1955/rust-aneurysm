# Yet Another Brainf\*\*k interpreter

An interpreter for `.bf` files (or any file that is written for the Brainf\*\*k _programming_ language) that is written in the Rust programming language and is designed with efficiency in mind

## What is Brainf\*\*k anyways?

According to [esolangs.org](https://esolangs.org/wiki/Main_Page):

> Brainfuck is one of the most famous esoteric programming languages, and has inspired the creation of a host of other languages.

Brainfuck operates on an array of memory cells, each initially set to zero. In most implementations, the array is 30,000 cells long, but this can be configured by using the `-m --mem` flag.

There is a pointer, initially pointing to the first memory cell. There are 8 commands, `><+-.,[]` (all other characters are considered comments), which involve around moving the pointer, manipulating the memory cell at the pointer's location, reading and writing from/to a source/sink (stdin/stdout) and implementing a `jump`-like behaviour.

**IMPORTANT:** unmatched loop brackets will be considered a syntax error (<https://brainfuck.org/brainfuck.html>)

For more info, check <https://esolangs.org/wiki/Brainfuck>

## Installing / Compiling

If you have Cargo installed, to install the most recent version from [crates.io](https://crates.io/), simply run `cargo install aneurysm`. If you want to install the latest beta from GitHub, run `cargo install --git https://github.com/Oakchris1955/rust-aneurysm.git --branch beta`. If you have already installed this package and want to update it to the latest version, run the corresponding command with the `--force` flag

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

### About CPU and memory usage

This program basically adheres to the DOTADIW (Do One Thing and Do It Well) principle: in other words, if you run a program that never terminates, it could eat up your CPU. The same goes when you set its memory usage to an abnormal number (although in that case, the OS will probably terminate the process, see Linux's case: [Out Of Memory Management](https://www.kernel.org/doc/gorman/html/understand/understand016.html)). This program puts trust in the user, so that it can DOTADIW.

## TODO

- [x] Add generics for the `WrappingUInt` struct, rename it to `Modular` and move it to its own submodule
- [ ] Separate crate for `modular` submodule
- [ ] (Work in progress) Debugger (~~preferable some kind of remote server that can be used with existing debuggers~~ not possible/realistic, designing own debugger from scratch)
- [ ] create own shell (shellfish is good, but it doesn't exactly allows us to add aliases...)
- [x] correctly handle loops in `aneurysm_lib`
- [ ] Find a better prompt string for `lobotomy`
- [ ] Docs for `aneurysm_lib`
- [ ] Compiler?

## License

[MIT](LICENSE)
