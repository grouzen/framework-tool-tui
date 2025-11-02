pub enum AdjustableControl {
    Percentage(bool, u8),
    Range(bool, f32, f32),
}

impl AdjustableControl {
    pub fn toggle_focus(&self) -> Self {
        match self {
            AdjustableControl::Percentage(focused, value) => {
                AdjustableControl::Percentage(!focused, *value)
            }
            AdjustableControl::Range(focused, from, to) => {
                AdjustableControl::Range(!focused, *from, *to)
            }
        }
    }

    pub fn is_focused(&self) -> bool {
        match self {
            AdjustableControl::Percentage(focused, _) => *focused,
            AdjustableControl::Range(focused, _, _) => *focused,
        }
    }

    pub fn get_percentage_value(&self) -> Option<u8> {
        match self {
            AdjustableControl::Percentage(_, value) => Some(*value),
            _ => None,
        }
    }
}

pub fn percentage_control(value: u8) -> AdjustableControl {
    AdjustableControl::Percentage(false, value)
}

pub fn range_control(from: f32, to: f32) -> AdjustableControl {
    AdjustableControl::Range(false, from, to)
}
