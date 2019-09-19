pub const VERSION: &str = "v1.0.0";
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
        start_input,
    },
    screen::
    {
        change_screen,
    },
    time::
    {
        start_time_loop,
    }
};

// Start of the program
fn main() 
{
    let args = check_args();
    change_screen();
    start_input();
    start_time_loop(&args);
}