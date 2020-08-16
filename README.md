# Simple command line tool
Print user info from a Json file

USAGE:
- ```print-users <INPUT> [LIMIT]```

ARGS:
- ```<INPUT>```    Sets the input user file to use
- ```<LIMIT>```    The number of how many users will be printed. If it is not provided, all users will be printed.

FLAGS:
- ```-h, --help```       Prints help information
- ```-V, --version```    Prints version information

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
## display help info
./target/debug/print-users --help 

## read from data.json and only print 1 user
./target/debug/print-users /path/data.json  1

## read from data.json and print all users
./target/debug/print-users /path/data.json
```
## Sample input file 

```
[{
	"name": "tony",
	"age": 26

},{
	"name": "tom",
	"age": 26
}]
```




