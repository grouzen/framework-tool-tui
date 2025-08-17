// pub trait AdjustableControl {
//     fn toggle_focus(&mut self);

//     fn is_focused(&self) -> bool;
// }

// pub struct PercentageControl {
//     focused: bool,
//     value: u8,
// }

// impl PercentageControl {
//     pub fn new() -> Self {
//         Self {
//             focused: false,
//             value: 0,
//         }
//     }
// }

// impl AdjustableControl for PercentageControl {
//     fn toggle_focus(&mut self) {
//         self.focused = !self.focused;
//     }

//     fn is_focused(&self) -> bool {
//         self.focused
//     }
// }

// pub struct RangeControl {
//     focused: bool,
//     from: f32,
//     to: f32,
// }

// impl RangeControl {
//     pub fn new() -> Self {
//         Self {
//             focused: false,
//             from: 0.0,
//             to: 0.0,
//         }
//     }
// }

// impl AdjustableControl for RangeControl {
//     fn toggle_focus(&mut self) {
//         self.focused = !self.focused;
//     }

//     fn is_focused(&self) -> bool {
//         self.focused
//     }
// }

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
            AdjustableControl::Percentage(focused, ..) => *focused,
            AdjustableControl::Range(focused, ..) => *focused,
        }
    }

    pub fn get_percentage_value(&self) -> Option<u8> {
        match self {
            AdjustableControl::Percentage(.., value) => Some(*value),
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
