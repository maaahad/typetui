use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use ratatui::{DefaultTerminal, Frame};
use std::io::stdout;

fn main() -> color_eyre::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::Grey),
        Print("First style with crossterm"),
        ResetColor
    )?;

    color_eyre::install()?;
    ratatui::run(app)?;

    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello world!!!", frame.area());
}
