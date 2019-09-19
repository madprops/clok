use crate::
{
    screen::exit
};

use std::
{
    thread,
    io::stdin,
    sync::
    {
        Arc,
        atomic::
        {
            AtomicBool, Ordering,
        },
    }
};

use termion::input::TermRead;

// Start input detection
pub fn start_input()
{
    // If any key is detected
    // exit the program
    thread::spawn(move || 
    {
        let stdin = stdin();

        let _ = match stdin.events().next()
        {
            Some(_) =>
            {
                exit();
            }
            None => ()
        };
    });

    // Detect ctrl+c 
    // to exit the program
    thread::spawn(move || 
    {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).unwrap();

        while running.load(Ordering::SeqCst) {}
        exit();
    });
}