use crate::
{
    s,
};

use std::str;
use termion::color;
use figlet_rs::FIGfont;

// Return a fig string
pub fn get_string(s: &str) -> String
{
    let font = FIGfont::from_content(str::from_utf8(include_bytes!("../colossal.flf")).unwrap());
    let fig = font.convert(s).unwrap().to_string();
    let s = s!(fig.replace("$", " ").trim_end()); s
}

// Get the width and height of a string
pub fn get_s_dimensions(s: &str) -> (u16, u16)
{
    let mut w = 0;
    let mut h = 0;

    for line in s.lines()
    {
        h += 1;
        let len = line.len();
        if len > w {w = len}
    }

    (w as u16, h as u16)
}

// Get a color from a keyword
pub fn get_color(c: String, fallback: &str) -> String
{
    match &c.to_lowercase()[..]
    {
        "red" => format!("{}", color::Fg(color::Red)),
        "lightred" => format!("{}", color::Fg(color::LightRed)),
        "green" => format!("{}", color::Fg(color::Green)),
        "lightgreen" => format!("{}", color::Fg(color::LightGreen)),
        "blue" => format!("{}", color::Fg(color::Blue)),
        "lightblue" => format!("{}", color::Fg(color::LightBlue)),
        "yellow" => format!("{}", color::Fg(color::Yellow)),
        "lightyellow" => format!("{}", color::Fg(color::LightYellow)),
        "cyan" => format!("{}", color::Fg(color::Cyan)),
        "lightcyan" => format!("{}", color::Fg(color::LightCyan)),
        "magenta" => format!("{}", color::Fg(color::Magenta)),
        "lightmagenta" => format!("{}", color::Fg(color::LightMagenta)),
        "black" => format!("{}", color::Fg(color::Black)),
        "lightblack" => format!("{}", color::Fg(color::LightBlack)),
        "white" => format!("{}", color::Fg(color::White)),
        "lightwhite" => format!("{}", color::Fg(color::LightWhite)),
        _ => 
        {
            if fallback.is_empty()
            {
                format!("{}", color::Fg(color::Blue))
            }

            else
            {
                return get_color(s!(fallback), "");
            }
        }
    }
}