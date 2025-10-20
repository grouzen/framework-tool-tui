use framework_tool_tui::app::App;
use uzers::get_current_uid;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if !check_permissions() {
        return Err(color_eyre::Report::msg(
            "The application needs to be run with root privileges.",
        ));
    }

    let mut terminal = ratatui::init();
    let mut app = App::new()?;

    let result = app.run(&mut terminal).await;

    ratatui::restore();

    result
}

fn check_permissions() -> bool {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let is_admin = get_current_uid() == 0;

    is_admin
}
