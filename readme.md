### readc

Reads a single character from stdin and outputs its numeric code, with a timeout that exits the program if it is reached. The timeout value is mandatory and must be passed as milliseconds.

**Usage:**

```
$ readc 1500
```

This will read from stdin for 1500 milliseconds; if a character is received, the program immediately prints its numeric code and exits, or if no character is received within the allotted 1500 milliseconds, the program prints -1 and exits.

The program uses `ncurses` to read input, and clears the screen while polling input. If the program is used in a subcommand this is avoided.

### License

MIT license
