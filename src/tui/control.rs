pub enum AdjustableControl {
    Percentage(bool, u8),
    Range(bool, usize, usize),
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
            AdjustableControl::Percentage(focused, ..) => *focused,
            AdjustableControl::Range(focused, ..) => *focused,
        }
    }
}

pub fn percentage_control(value: u8) -> AdjustableControl {
    AdjustableControl::Percentage(false, value)
}

pub fn range_control(from: usize, to: usize) -> AdjustableControl {
    AdjustableControl::Range(false, from, to)
}
