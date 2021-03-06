//! Tiny crates that stops the console window form closing when the program finishes.
use std::io;
use std::io::Write;

// Flush only when possible.
fn try_flush() {
    io::stdout().flush().unwrap_or(())
}

/// Using the `enter_to_continue` module makes is the simplest way of using this crate, however, the only key you can use with it is the enter key.
pub mod enter_to_continue {
    use crate::try_flush;
    use std::io;
    /// ### Message then close with enter.
    /// Prompts user with message `"Press enter to close."`, waits for the user to press enter then ends to program (closing the window).
    /// Add
    /// ```no_run
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and

    /// ```no_run
    /// dont_disappear::enter_to_continue::default();
    /// ```
    /// to where your program ends
    pub fn default() {
        custom_msg("Press enter to exit.");
    }
    /// ### Custom message then close with enter.
    /// Prompts user with a custom message, waits for the user to press enter then ends to program (closing the window).
    /// Add
    /// ```no_run
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and

    /// ```no_run
    /// dont_disappear::enter_to_continue::custom_msg("Your custom message.");
    /// ```
    /// to where your program ends
    pub fn custom_msg(msg: &str) {
        print!("{}", msg);
        try_flush();
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
/// The `any_key_to_continue` module responds to any key press, however, can return strange characters when Ctrl-c or Delete keys are used.
pub mod any_key_to_continue {
    use crate::try_flush;
    use crossterm::input;
    /// ### Message then close with any key.
    /// Prompts user with message `"Press any key to continue"`, waits for the user to press a key then ends to program (closing the window).
    /// Add
    /// ```rust
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and

    /// ```no_run
    /// dont_disappear::any_key_to_continue::default();
    /// ```
    /// to where your program ends
    pub fn default() {
        custom_msg("Press any key to continue")
    }
    /// ### Custom message then close with any key.
    /// Prompts user with a custom message, waits for the user to press a key then ends to program (closing the window).
    /// Add
    /// ```rust
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and

    /// ```no_run
    /// dont_disappear::any_key_to_continue::custom_msg("Your custom message.");
    /// ```
    /// to where your program ends
    #[allow(unused_must_use)]
    pub fn custom_msg(msg: &str) {
        print!("{}", msg);
        try_flush();
        input().read_char();
    }
}

/// ### Press close only with window manager or Ctrl-c.
/// The program just stops and waits to be killed by pressing close only with window manager or Ctrl-c.
/// The thread is [parked](https://doc.rust-lang.org/nightly/std/thread/fn.park.html) so it does not use CPU.
/// Add
/// ```no_run
/// extern crate dont_disappear;
/// ```
/// to the top of your file
/// and
/// ```no_run
/// dont_disappear::press_close();
/// ```
/// to where your program ends
// possible
pub fn press_close() {
    loop {
        std::thread::park();
    }
}
