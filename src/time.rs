use crate::
{
    s, p,
    DEF_HOURS_COLOR,
    DEF_MINUTES_COLOR,
    DEF_PM_COLOR,
    TIME_LOOP_DELAY,
    utils::
    {
        get_string,
        get_s_dimensions,
        get_color,
    },
    args::Args,
};

use std::
{
    thread, 
    cmp::max,
    time::Duration,
};

use chrono::prelude::*;

use termion::
{
    color, cursor, clear,
};

// Start the loop to update the clock
pub fn start_time_loop(args: Args)
{
    let mut last_mins: i8 = -1;

    loop
    {
        let now = Local::now();
        let mins = now.minute() as i8;

        if mins != last_mins
        {
            last_mins = mins;
            update_time(&args, now);
        }

        let sd = Duration::from_millis(TIME_LOOP_DELAY);
        thread::sleep(sd);
    }
}

// Update the clock
pub fn update_time(args: &Args, now: DateTime<Local>)
{
    let size = termion::terminal_size().unwrap();
    let mins = now.minute();
    let mut pm = false; 
    let hour;

    if args.a24_hours
    {
        hour = now.hour();
    }

    else
    {
        let h = now.hour12();
        pm = h.0; hour = h.1;
    }

    let h = get_string(&hour.to_string());
    let m = get_string(&format!("{:02}", mins));
    let hd = get_s_dimensions(&h);
    let md = get_s_dimensions(&m);
    let mut d = s!(""); let dd;
    let height;

    if args.a24_hours || args.no_pm
    {
        height = max(hd.1, md.1);
    }

    else
    {
        if args.lowercase
        {
            d = get_string(if pm {"pm"} else {"am"});
        }

        else
        {
            d = get_string(if pm {"PM"} else {"AM"});
        }

        dd = get_s_dimensions(&d);
        height = max(max(hd.1, md.1), dd.1);
    }

    // Clear all
    p!("{}", clear::All);

    // Print the hour
    for line in h.lines()
    {
        p!(format!("{}{}{}{}",
            cursor::Right(4),
            get_color(s!(args.hours_color), DEF_HOURS_COLOR), 
            line, color::Fg(color::Reset)));
    }

    // Fix to adjust vertical padding
    let diff = height - hd.1;

    for _ in 0..diff
    {
        p!("");
    }    

    // Print the minutes
    for (i, line) in m.lines().enumerate()
    {
        p!(format!("{}{}{}{}", 
            cursor::Goto(hd.0 + 8, size.1 - height + i as u16), 
            get_color(s!(args.minutes_color), DEF_MINUTES_COLOR), 
            line, color::Fg(color::Reset)));
    }

    // Print am/pm
    if !args.a24_hours && !args.no_pm
    {
        for (i, line) in d.lines().enumerate()
        {
            p!(format!("{}{}{}{}", 
                cursor::Goto(hd.0 + md.0 + 12, size.1 - height + i as u16), 
                get_color(s!(args.pm_color), DEF_PM_COLOR), 
                line, color::Fg(color::Reset)));
        }
    }

    // If the show date flag is passed
    // then show the date below
    if args.show_date
    {
        p!(format!("\n{}{}", 
            cursor::Right(4), 
            now.date().format("%A %e of %B")));
    }
}