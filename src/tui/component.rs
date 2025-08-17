use ratatui::{Frame, layout::Rect};

use crate::{app::AppEvent, framework::FrameworkControls};

pub mod brightness_panel;
pub mod charge_panel;
pub mod footer;
pub mod main;
pub mod privacy_panel;
pub mod smbios_panel;
pub mod title;

pub trait Component {
    fn handle_input(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        Ok(None)
    }

    fn render(&mut self, _frame: &mut Frame, _area: Rect, _controls: &FrameworkControls) {}
}
