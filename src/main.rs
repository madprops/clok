pub const VERSION: &str = "v1.2.0";
pub const DEF_HOURS_COLOR: &str = "green";
pub const DEF_MINUTES_COLOR: &str = "blue";
pub const DEF_PM_COLOR: &str = "red";
pub const TIME_LOOP_DELAY: u64 = 1000;

mod macros;
mod args;
mod input;
mod screen;
mod utils;
mod time;

use crate::
{
    args::check_args,
    input::
    {
        start_ctrlc,
    },
    screen::
    {
        change_screen,
    },
};

// Start of the program
fn main() 
{
    let args = check_args();
    change_screen();
    start_ctrlc(args);
}