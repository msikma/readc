## `readc`

Reads a single character from stdin and outputs its numeric code (wrapped in underscores) with a timeout that exits the program if it is reached. The timeout value is mandatory and must be passed as milliseconds.

**Usage:**

```
$ readc 1500
```

This will read from stdin for 1500 milliseconds; if a character is received, the program immediately prints its numeric code and exits, or if no character is received within the allotted 1500 milliseconds, the program prints -1 and exits.

### Output problems

The program uses `ncurses` to read input, and clears the screen while polling input. If the program is used in a subcommand this is avoided; but the output will still contain extra escape sequences. This is why the output is wrapped in underscores, so that it can easily be separated from the rest, e.g. this way, using *enter* as the test key:

```bash
$ a=`./target/release/readc 1500 | grep -o '_-\?[0-9]\+_'`
$ echo "The result is: $a"
The result is: _10_
```

This is very hackish. If anyone has a better idea on how to do this, please let me know by raising an issue.

## License

MIT license
