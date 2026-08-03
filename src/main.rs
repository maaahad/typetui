use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph, Widget},
};
use std::io::stdout;

struct Grid {
    cols: usize,
    rows: usize,
}

impl Widget for Grid {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(1));

        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            Paragraph::new(format!("Area {:02}", i + 1))
                .block(Block::bordered())
                .render(cell, buf);
        }
    }
}

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
    // frame.render_widget(Block::bordered().title("Type Tui"), frame.area());
    frame.render_widget(Grid { cols: 5, rows: 4 }, frame.area());
}
