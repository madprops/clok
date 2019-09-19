use crate::
{
    screen::exit,
    time::start_time_loop,
    args::Args,
};

use std::thread;
use chan::chan_select;
use chan_signal::Signal;

// Detect ctrl+c 
// to exit the program
pub fn start_ctrlc(args: Args)
{
    // Signal gets a value when the OS sent a INT or TERM signal.
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    thread::spawn(move || 
    {
        start_time_loop(args);
    });

    // Wait for a signal or for work to be done.
    chan_select! 
    {
        signal.recv() -> _ => 
        {
            exit();
        }
    }
}