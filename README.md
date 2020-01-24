# pwutils

A collection of password utilities.

## Building from source (dev)

1. Clone the repository 
```shell
$ git clone https://github.com/rhgrieve/pwutils.git
$ cd pwutils
```
2. Build it 
```shell
$ cargo build
```

3. Run it 
```shell
$ cd target/debug
$ ./pwutils <options>
```

## Subcommands

### gen
generates secure passwords

```shell
pwutils.exe-gen 0.1.0

USAGE:
    pwutils.exe gen [FLAGS] [OPTIONS]

FLAGS:
    -c, --copy       automatically copy generated password to clipboard
    -h, --help       Prints help information
    -s, --save       hashes and saves the password to .txt file
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>    set password length [default: 8]
```

### check
checks password quality

```shell
pwutils.exe-check 0.1.0

USAGE:
    pwutils.exe check <password>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <password>
```

## TODO

- [ ] Saving hashed passwords
- [ ] Share password as secret
- [ ] Modify password rules as optional parameters
