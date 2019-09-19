use crate::
{
    p,
};

use std::
{
    process,
};

use termion::
{
    cursor, screen,
    terminal_size,
};

// Switch to the alternative screen
// Place the cursor at the bottom left
// and hide the cursor
pub fn change_screen()
{
    p!("{}", screen::ToAlternateScreen);
    let size = terminal_size().unwrap();
    p!(format!("{}{}", 
        cursor::Goto(1, size.1),
        cursor::Hide))
}

// Centralized function to exit the program
// Switches back to main screen before exiting
pub fn exit() -> !
{
    p!(format!("{}{}",
        cursor::Show,
        screen::ToMainScreen));

    process::exit(0)
}