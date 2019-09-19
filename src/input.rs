use crate::
{
    screen::exit,
};

use std::
{
    thread,
    sync::
    {
        Arc,
        atomic::
        {
            AtomicBool, Ordering,
        },
    }
};

// Detect ctrl+c 
// to exit the program
pub fn start_ctrlc()
{
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