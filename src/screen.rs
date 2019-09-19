use crate::
{
    p,
};

use std::
{
    process,
    io::{stdout, Write},
};

use termion::
{
    raw::IntoRawMode,
};

// Switch to the alternative screen
// Place the cursor at the bottom left
pub fn change_screen()
{
    p!("{}", termion::screen::ToAlternateScreen);
    let size = termion::terminal_size().unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, size.1)).unwrap();
}

// Centralized function to exit the program
// Switches back to main screen before exiting
pub fn exit() -> !
{
    p!(format!("{}{}", 
        termion::cursor::Show,
        termion::screen::ToMainScreen));

    process::exit(0)
}