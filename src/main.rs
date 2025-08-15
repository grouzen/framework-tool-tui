mod app;
mod framework;
use app::App;

fn main() -> Result<(), anyhow::Error> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
