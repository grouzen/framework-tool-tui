use framework_tool_tui::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::new();

    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}
