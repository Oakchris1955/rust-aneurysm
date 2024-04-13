# Yet Another Brainf**k interpreter

An interpreter for `.bf` files (or any file that is written for the Brainf\*\*k *programming* language) that is written in the Rust programming language and is designed with efficiency in mind

## Installing / Compiling

(Note: make sure you have `git` and the `cargo` tool installed before doing this)
Clone this repository with `git`, `cd` to the cloned repo directory and run the `build.sh` script to create an exewcutable in the repo's main directory

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
