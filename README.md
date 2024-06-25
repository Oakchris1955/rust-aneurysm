# Yet Another Brainf\*\*k interpreter

An interpreter & a debugger for `.bf` files (or any file that is written for the Brainf\*\*k _programming_ language) that are written in the Rust programming language and are designed with efficiency and easy of usage in mind

## What is Brainf\*\*k anyways?

According to [esolangs.org](https://esolangs.org/wiki/Main_Page):

> Brainfuck is one of the most famous esoteric programming languages, and has inspired the creation of a host of other languages.

Brainfuck operates on an array of memory cells, each initially set to zero. In most implementations, the array is 30,000 cells long, but this can be configured by using the `-m --mem` flag.

There is a pointer, initially pointing to the first memory cell. There are 8 commands, `><+-.,[]` (all other characters are considered comments), which involve around moving the pointer, manipulating the memory cell at the pointer's location, reading and writing from/to a source/sink (stdin/stdout) and implementing a `jump`-like behaviour.

**IMPORTANT:** unmatched loop brackets will be considered a syntax error (<https://brainfuck.org/brainfuck.html>)

For more info, check <https://esolangs.org/wiki/Brainfuck>

## Installing / Compiling

If you have Cargo installed, to install the most recent version from [crates.io](https://crates.io/), simply run `cargo install aneurysm`. If you want to install the latest version from GitHub, run `cargo install --git https://github.com/Oakchris1955/rust-aneurysm.git`. If you have already installed this package and want to update it to the latest version, run the corresponding command with the `--force` flag

## Usage

This crate consists of 3 targets, 2 binaries (`aneurysm` and `lobotomy`) and a library (also named `aneurysm`). While one could technically use the library to write their own interpreter, the API is currently unstable and thus such usage is discouraged, at least until the 1.0.0 release (if such thing ever occurs). Apart from that, bloat dependencies that aren't necessary for the `aneurysm lib` and are only used by the binaries (such as `clap`) will be installed

### Aneurysm

The Brainf\*\*k interpreter

All the functionality is exposed to the user in the forms of CLI arguments (the only exception to this is [logging](#logging) )

#### CLI

```text
A Brainf**k interpreter written in Rust with minimal dependencies

**Usage**: aneurysm [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  Brainf**k file to execute [default: main.bf]

Options:
  -m, --mem <memory>  The memory size in bytes/cells to allocate for the program [default: 30000]
  -v, --verbose       Enable verbose logging
  -e, --echo          Whether or not to echo characters written to stdin
  -h, --help          Print help
  -V, --version       Print version
```

#### Logging

Verbose logging will be printed to the stderr when the `-v --verbose` flag is set. Anything with a level of `INFO` or above will be printed, or `DEBUG` is the program is run with debug assertations on. If the flag isn't set, the default level will be `WARN`. Please note that you can set the logging level at runtime using the `RUST_LOG` environment variable, which will take precedence over the above

### Lobotomy

A debugger for Brainf\*\*k programs

Unlike [`aneurysm`](#aneurysm), the CLI only serves minimal functionality, such as selecting which file to start debugging. `lobotomy` is a shell itself and the rest of the functionality is exposed through embedded commands in this shell, such as `run`, `memdump` and `breakpoint`. Running `help` should be enough to get started. From there, use the commands as you would on any POSIX shell

#### CLI

```text
A debugger for Brainf**k programs

Usage: lobotomy <FILENAME>

Arguments:
  <FILENAME>  Path to the file to debug

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Logging

Most of the logs won't be shown to the console (only anything with a level of `WARN` or above will be logged to the stderr) and everything else will be logged to a file inside a directory under the `${data_local_dir}/logs` of the project according to the [directories](https://crates.io/crates/directories) crate. This directory is (for the 3 most widespread OS families):

| Linux                                                                | Windows                                                                                         | MacOS                                             |
| -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| \$XDG_DATA_HOME/lobotomy/logs/ or \$HOME/.local/share/lobotomy/logs/ | {FOLDERID_LocalAppData}/lobotomy/data/logs/ ({%USERPROFILE%/AppData/Local}/lobotomy/data/logs/) | \$HOME/Library/Application Support/lobotomy/logs/ |

The `RUST_LOG` variable affects the logging level of what should be written in the file. If it isn't provided, the default level is set to `INFO`

### About CPU and memory usage

These programs adhere to the DOTADIW (Do One Thing and Do It Well) principle: in other words, if you run a BF program that never terminates, it could eat up your CPU. The same goes when you set its memory usage to an abnormal number (although in that case, the OS will probably terminate the process, see Linux's case: [Out Of Memory Management](https://www.kernel.org/doc/gorman/html/understand/understand016.html)). This crate puts trust in the user, so that it can DOTADIW.

## TODO

- [x] Add generics for the `WrappingUInt` struct, rename it to `Modular` and move it to its own submodule
- [ ] Separate crate for `modular` submodule
- [ ] (Work in progress) Debugger (~~preferable some kind of remote server that can be used with existing debuggers~~ not possible/realistic, designing own debugger from scratch)
- [ ] create own shell (shellfish is good, but it doesn't exactly allows us to add aliases...)
- [x] update README: add debugger info
- [x] correctly handle loops in `aneurysm_lib`
- [ ] Find a better prompt string for `lobotomy`
- [ ] Docs for `aneurysm_lib`
- [ ] Compiler?

## License

[MIT](LICENSE)
