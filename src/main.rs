use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{Result, stdout};

fn main() -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::Grey),
        Print("First style with crossterm"),
        ResetColor
    )?;

    Ok(())
}
