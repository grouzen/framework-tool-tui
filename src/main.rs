use std::io;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Direction, Constraint},
};

fn main() -> Result<(), anyhow::Error> {
    // Terminal init
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_title = "FRAMEWORK SYSTEM TUI";
    let footer = "[Tab] Switch Focus  [Enter] Apply  [Esc] Cancel  [Q] Quit";

    // Main draw loop - single frame only (static placeholder layout)
    terminal.draw(|f| {
        let size = f.size();

        // Overall layout split: Vertical: Title, Panels, Footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Title
                Constraint::Min(0),      // Panels main content
                Constraint::Length(2),   // Footer
            ])
            .split(size);

        // Title
        let title_block = Block::default()
            .title(app_title)
            .borders(Borders::ALL);
        f.render_widget(title_block, chunks[0]);

        // Panels grid: 3 rows of 3 columns
        let panel_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(chunks[1]);

        // First Row
        let first_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
            .split(panel_chunks[0]);
        let battery_panel = Block::default().title("BATTERY & CHARGING").borders(Borders::ALL);
        let fan_panel = Block::default().title("FAN CONTROL").borders(Borders::ALL);
        let usb_panel = Block::default().title("USB PD PORTS").borders(Borders::ALL);
        f.render_widget(battery_panel, first_row[0]);
        f.render_widget(fan_panel, first_row[1]);
        f.render_widget(usb_panel, first_row[2]);

        // Second Row
        let second_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
            .split(panel_chunks[1]);
        let privacy_panel = Block::default().title("PRIVACY CONTROLS").borders(Borders::ALL);
        let input_panel = Block::default().title("INPUT DECK MODE").borders(Borders::ALL);
        let lighting_panel = Block::default().title("KEYBOARD LIGHTING").borders(Borders::ALL);
        f.render_widget(privacy_panel, second_row[0]);
        f.render_widget(input_panel, second_row[1]);
        f.render_widget(lighting_panel, second_row[2]);

        // Third Row
        let third_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
            .split(panel_chunks[2]);
        let fingerprint_panel = Block::default().title("FINGERPRINT BRIGHTNESS").borders(Borders::ALL);
        let sysinfo_panel = Block::default().title("SYSTEM INFO").borders(Borders::ALL);
        let log_panel = Block::default().title("MESSAGES / LOG").borders(Borders::ALL);
        f.render_widget(fingerprint_panel, third_row[0]);
        f.render_widget(sysinfo_panel, third_row[1]);
        f.render_widget(log_panel, third_row[2]);

        // Footer
        let footer_paragraph = Paragraph::new(footer)
            .centered()
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer_paragraph, chunks[2]);
    })?;

    // Cleanup
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}
