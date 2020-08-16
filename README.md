# Simple command line tool
Print user info from a Json file

USAGE:
    print-users <INPUT> [LIMIT]

ARGS:
    <INPUT>    Sets the input user file to use
    <LIMIT>    The number of how many users will be printed. If it is not provided, all users will be printed.

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Build project:

```bash
cargo build 
```

## Run

```bash
./target/debug/print-users --help 
./target/debug/print-users /path/data.json  1  
```





