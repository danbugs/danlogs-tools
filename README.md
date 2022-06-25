# danlogs Tools

This is a repository to group tools for making the process of creating videos for my YouTube channel (i.e., danlogs) easier.

## danlogs Script Lexer and Splitter (dsls)

```sh
❯ dsls --help
danlogs-tools 0.1.0
Dan Chiarlone
A lexer and splitter (i.e., splitting between script and code files) for the stuff I write while
preparing videos for my YouTube channel — danlogs

USAGE:
    dsls --file-to-parse <FILE_TO_PARSE>

OPTIONS:
    -f, --file-to-parse <FILE_TO_PARSE>    
    -h, --help                             Print help information
    -V, --version                          Print version information
```

### How to Install dsls?

If, for whatever reason, you want to install DSLS, you can do that by:

```sh
❯ git clone https://github.com/danbugs/danlogs-tools.git
❯ make build
❯ make install
```

> Note: `make install` uses the `install` command, which is a Unix thing. If you are on Windows, you'll probably just have to move it to your any specific directory you want and then add that directory to your PATH.