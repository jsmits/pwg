# pwg

[![CI](https://github.com/jsmits/pwg/actions/workflows/ci.yml/badge.svg)](https://github.com/jsmits/pwg/actions/workflows/ci.yml)

A password generator for the command-line

## Usage

```
$ pwg
```

#### Available arguments

```
$ pwg --help

pwg 0.1.0
A password generator for the command-line

USAGE:
    pwg [FLAGS] [OPTIONS]

FLAGS:
        --exclude-lowercase    Exclude lowercase letters
        --exclude-numbers      Exclude numbers
        --exclude-similar      Exclude similar characters
        --exclude-symbols      Exclude symbols
        --exclude-uppercase    Exclude uppercase letters
    -h, --help                 Prints help information
        --spaces               Include spaces
    -V, --version              Prints version information

OPTIONS:
    -l, --length <length>    Password length [default: 21]
```

## Build

```
$ cargo build --release
```