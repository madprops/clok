use crate::
{
    s, VERSION
};

use clap::
{
    App, Arg,
};

// Starts the argument system
pub fn check_args() -> Args
{
    let matches = App::new("clok")
    .version(VERSION)
    .about("You know what time it is. Color arguments can include: black, white, red, blue, green, yellow, cyan, magenta, or their light derivate: for example lightred.")
    .arg(Arg::with_name("hours_color")
        .long("hours-color")
        .multiple(false)
        .help("Color for the hours")
        .takes_value(true))
    .arg(Arg::with_name("minutes_color")
        .long("minutes-color")
        .multiple(false)
        .help("Color for the minutes")
        .takes_value(true))
    .arg(Arg::with_name("pm_color")
        .long("pm-color")
        .multiple(false)
        .help("Color for AM/PM")
        .takes_value(true))
    .arg(Arg::with_name("24_hours")
        .long("24-hours")
        .multiple(false)
        .help("Use 24 hours instead of 12"))
    .arg(Arg::with_name("lowercase")
        .long("lowercase")
        .multiple(false)
        .help("Use am/pm instead of AM/PM"))
    .arg(Arg::with_name("no_pm")
        .long("no-pm")
        .multiple(false)
        .help("Don't show AM/PM"))        
    .get_matches();

    let hours_color = if let Some(x) = matches.value_of("hours_color")
    {s!(x.trim())} else {s!("")};

    let minutes_color = if let Some(x) = matches.value_of("minutes_color")
    {s!(x.trim())} else {s!("")};

    let pm_color = if let Some(x) = matches.value_of("pm_color")
    {s!(x.trim())} else {s!("")};

    let a24_hours = matches.occurrences_of("24_hours") > 0;
    let lowercase = matches.occurrences_of("lowercase") > 0;
    let no_pm = matches.occurrences_of("no_pm") > 0;

    Args
    {
        hours_color, minutes_color, 
        pm_color, a24_hours, lowercase, no_pm
    }
}

// Arguments struct
#[derive(Debug)]
pub struct Args
{
    pub hours_color: String,
    pub minutes_color: String,
    pub pm_color: String,
    pub a24_hours: bool,
    pub lowercase: bool,
    pub no_pm: bool,
}