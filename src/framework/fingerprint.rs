use color_eyre::eyre::Report;
use framework_lib::chromium_ec::{commands::FpLedBrightnessLevel, CrosEc};

use crate::framework::EcErrorWrapper;

const FP_LED_BRIGHTNESS_LEVEL_LOW: u8 = 15;
const FP_LED_BRIGHTNESS_LEVEL_MEDIUM: u8 = 40;
const FP_LED_BRIGHTNESS_LEVEL_HIGH: u8 = 55;

pub enum FpLedBrightnessCapability {
    Level,
    Percentage,
}

pub struct Fingerprint {
    pub led_brightness_capability: FpLedBrightnessCapability,
}

impl Fingerprint {
    pub fn new(ec: &CrosEc) -> color_eyre::Result<Self> {
        let led_capability = match ec
            .get_fp_led_level()
            .map_err(|error| Report::from(EcErrorWrapper(error)))?
        {
            (_, Some(_)) => FpLedBrightnessCapability::Percentage,
            (_, None) => FpLedBrightnessCapability::Level,
        };

        Ok(Self {
            led_brightness_capability: led_capability,
        })
    }

    pub fn percentage() -> Self {
        Self {
            led_brightness_capability: FpLedBrightnessCapability::Percentage,
        }
    }

    pub fn level() -> Self {
        Self {
            led_brightness_capability: FpLedBrightnessCapability::Level,
        }
    }

    pub fn adjust_led_brightness_by_delta(&self, current: u8, delta: i8) -> u8 {
        match self.led_brightness_capability {
            FpLedBrightnessCapability::Level => {
                let level = led_brightness_percentage_to_level(current);
                let new_level = adjust_led_brightness_level_by_delta(level, delta);

                led_brightness_level_to_percentage(current, new_level)
            }
            FpLedBrightnessCapability::Percentage => {
                let new_value = current as i8 + delta;

                // NOTE: disable setting the FP brightness to less than 5%
                if new_value < 5 {
                    current
                } else {
                    new_value as u8
                }
            }
        }
    }
}

pub fn led_brightness_percentage_to_level(value: u8) -> FpLedBrightnessLevel {
    if value <= FP_LED_BRIGHTNESS_LEVEL_LOW {
        FpLedBrightnessLevel::Low
    } else if value <= FP_LED_BRIGHTNESS_LEVEL_MEDIUM {
        FpLedBrightnessLevel::Medium
    } else {
        FpLedBrightnessLevel::High
    }
}

pub fn led_brightness_percentage_to_level_name(value: u8) -> &'static str {
    if value <= FP_LED_BRIGHTNESS_LEVEL_LOW {
        "Low"
    } else if value <= FP_LED_BRIGHTNESS_LEVEL_MEDIUM {
        "Medium"
    } else {
        "High"
    }
}

fn led_brightness_level_to_percentage(current: u8, level: FpLedBrightnessLevel) -> u8 {
    match level {
        FpLedBrightnessLevel::Low => FP_LED_BRIGHTNESS_LEVEL_LOW,
        FpLedBrightnessLevel::Medium => FP_LED_BRIGHTNESS_LEVEL_MEDIUM,
        FpLedBrightnessLevel::High => FP_LED_BRIGHTNESS_LEVEL_HIGH,
        _ => current,
    }
}

fn adjust_led_brightness_level_by_delta(
    level: FpLedBrightnessLevel,
    delta: i8,
) -> FpLedBrightnessLevel {
    match level {
        FpLedBrightnessLevel::Low => {
            if delta > 0 {
                FpLedBrightnessLevel::Medium
            } else {
                FpLedBrightnessLevel::High
            }
        }
        FpLedBrightnessLevel::Medium => {
            if delta > 0 {
                FpLedBrightnessLevel::High
            } else {
                FpLedBrightnessLevel::Low
            }
        }
        FpLedBrightnessLevel::High => {
            if delta > 0 {
                FpLedBrightnessLevel::Low
            } else {
                FpLedBrightnessLevel::Medium
            }
        }
        _ => level,
    }
}

#[cfg(test)]
mod tests {

    use crate::framework::fingerprint::Fingerprint;

    #[test]
    fn adjust_led_brightness_by_delta_for_percentage() {
        let fingerprint = Fingerprint::percentage();

        let result_five = fingerprint.adjust_led_brightness_by_delta(10, -5);
        let result_six = fingerprint.adjust_led_brightness_by_delta(11, -5);
        let result_less_than_five = fingerprint.adjust_led_brightness_by_delta(9, -5);

        assert!(result_five == 5);
        assert!(result_six == 6);
        assert!(result_less_than_five == 9);
    }
}
