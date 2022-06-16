# pngme

Encode and decode secret messages in png files.
This project is my implementation of this [guide](https://picklenerd.github.io/pngme_book/).

## Compiling / running

compile:

```
cargo build
```

run:

```
cargo run <SUBCOMMAND>
```

## Installing

locally:

```
cargo install --path .
```

remotely:

```
cargo install --git https://github.com/DeathEyeXD/pngme
```

## Usage:

```
USAGE:
    pngme.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decode a secret message encoded in png file
    encode    Encode a chunk with given chunk type and message into file (note: by default it
                  creates given file if it doesnt exists, but if it exists it checks whether file is
                  a valid png)
    help      Print this message or the help of the given subcommand(s)
    print     Print png file data as bytes from given path
    remove    Remove (and decode) first secret message found with given chunk type encoded in
                  png file (note: it deletes most-recent message first, and use -a flag to delete
                  all matched messages)

```