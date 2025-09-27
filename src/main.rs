use framework_tool_tui::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::new();

    let result = app.run(&mut terminal).await;

    ratatui::restore();

    result
}
