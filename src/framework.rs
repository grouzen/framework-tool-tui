use std::sync::Arc;

use color_eyre::eyre::Report;
use framework_lib::chromium_ec::CrosEc;
use framework_lib::chromium_ec::CrosEcDriver;
use framework_lib::chromium_ec::EcError;
use framework_lib::smbios;

use crate::framework::fingerprint::led_brightness_percentage_to_level;
use crate::framework::fingerprint::Fingerprint;
use crate::framework::fingerprint::FpLedBrightnessCapability;
use crate::framework::info::FrameworkInfo;

pub mod fingerprint;
pub mod info;

// Copied from framework_lib::power
const EC_MEMMAP_FAN: u16 = 0x10; // Fan speeds 0x10 - 0x17
const EC_FAN_SPEED_ENTRIES: usize = 4;
/// Used on old EC firmware (before 2023)
const EC_FAN_SPEED_NOT_PRESENT: u16 = 0xFFFF;

pub struct Framework {
    ec: CrosEc,
    fingerprint: Arc<Fingerprint>,
}

#[derive(Debug)]
struct EcErrorWrapper(EcError);

impl std::fmt::Display for EcErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::error::Error for EcErrorWrapper {}

impl Framework {
    pub fn new(ec: CrosEc, fingerprint: Arc<Fingerprint>) -> Self {
        Framework { ec, fingerprint }
    }

    pub fn set_max_charge_limit(&self, value: u8) -> color_eyre::Result<()> {
        self.ec
            .set_charge_limit(0, value)
            .map_err(|error| Report::from(EcErrorWrapper(error)))
    }

    pub fn set_fp_brightness(&self, percentage: u8) -> color_eyre::Result<()> {
        match self.fingerprint.led_brightness_capability {
            FpLedBrightnessCapability::Level => {
                let level = led_brightness_percentage_to_level(percentage);

                self.ec
                    .set_fp_led_level(level)
                    .map_err(|error| Report::from(EcErrorWrapper(error)))
            }
            FpLedBrightnessCapability::Percentage => self
                .ec
                .set_fp_led_percentage(percentage)
                .map_err(|error| Report::from(EcErrorWrapper(error))),
        }
    }

    // NOTE: the underlying ec API is weird
    pub fn set_kb_brightness(&self, percentage: u8) {
        self.ec.set_keyboard_backlight(percentage);
    }

    pub fn get_info(&mut self) -> FrameworkInfo {
        let power = framework_lib::power::power_info(&self.ec);
        let charge_limit = self.ec.get_charge_limit().ok();
        let privacy = self.ec.get_privacy_info().ok();
        let fp_brightness = self.ec.get_fp_led_level().ok();
        let kb_brightness = self.ec.get_keyboard_backlight().ok();
        let smbios = smbios::get_smbios();
        let pd_ports = framework_lib::power::get_pd_info(&self.ec, 4)
            .into_iter()
            .map(Result::ok)
            .collect();
        let fan_rpm = self.get_fan_rpm().ok();

        FrameworkInfo::new(
            &power,
            &charge_limit,
            &privacy,
            &fp_brightness,
            kb_brightness,
            &smbios,
            pd_ports,
            fan_rpm,
        )
    }

    fn get_fan_rpm(&self) -> color_eyre::Result<Vec<u16>> {
        let fans = self
            .ec
            .read_memory(EC_MEMMAP_FAN, 0x08)
            .ok_or(Report::msg("Couldn't read fan info"))?;
        let mut rpms = Vec::new();

        for i in 0..EC_FAN_SPEED_ENTRIES {
            let rpm = u16::from_le_bytes([fans[i * 2], fans[1 + i * 2]]);

            if rpm == EC_FAN_SPEED_NOT_PRESENT {
                continue;
            }

            rpms.push(rpm);
        }

        Ok(rpms)
    }
}
