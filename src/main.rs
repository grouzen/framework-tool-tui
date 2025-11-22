use framework_tool_tui::app::App;
#[cfg(unix)]
use uzers::get_current_uid;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    check_permissions()?;

    let mut terminal = ratatui::init();
    let mut app = App::new()?;

    let result = app.run(&mut terminal).await;

    ratatui::restore();

    result
}

#[cfg(unix)]
fn check_permissions() -> color_eyre::Result<()> {
    let is_admin = get_current_uid() == 0;

    if !is_admin {
        return Err(color_eyre::Report::msg(
            "The application needs to be run with root privileges.",
        ));
    }

    Ok(())
}

#[cfg(windows)]
fn check_permissions() -> color_eyre::Result<()> {
    if let Err(err) = framework_lib::chromium_ec::CrosEc::new().version_info() {
        return Err(color_eyre::Report::msg(format!(
            "The application needs to be run as admin: {:?}.",
            err
        )));
    }

    Ok(())
}
