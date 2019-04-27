extern crate ncurses;
use ncurses::*;
use std::thread;
use std::time;
use std::env;
use std::str::FromStr;

const USAGE: &str = "Usage: readc [TIMEOUT_MS]";

/// Reads a single character from stdin with a timeout specified in milliseconds.
///
/// # Example
///
/// From the command line:
///
/// ```
/// $ readc 1500
/// ```
///
/// This will read a single character from stdin, and print its numeric value
/// and exit if it is received. If the user waits too long and the timeout
/// duration is reached, the program will exit with -1 as the result.
fn main() {
  // Grab command line arguments.
  let args: Vec<String> = env::args().collect();

  // Check to see if the user passed a timeout argument. If not, exit with usage message.
  if args.len() <= 1 {
    println!("{}", USAGE);
    return;
  }

  // Convert timeout to number. Exit if the user didn't pass a numeric value.
  let timeout = i32::from_str(&args[1]).unwrap_or(0);
  if timeout == 0 || timeout < 5 {
    println!("{}", USAGE);
    return;
  }

  // Start up ncurses to read for a keypress.
  let nc = initscr();
  refresh();
  // Don't block the read operation.
  nodelay(nc, true);

  let mut chr;
  let mut wait: i32 = 0;
  loop {
    wait += 5;
    thread::sleep(time::Duration::from_millis(5));
    chr = getch();
    if chr != -1 {
      break;
    }
    if wait >= timeout {
      break;
    }
  }
  
  // Terminate ncurses and print our character if we have it (or -1).
  endwin();
  print!("{}", chr);
}
