use framework_lib::chromium_ec::commands::FpLedBrightnessLevel;
use framework_lib::power::PowerInfo;
use framework_lib::power::UsbPdPowerInfo;
use framework_lib::smbios;
use smbioslib::DefinedStruct;
use smbioslib::SMBiosData;

#[derive(Default)]
pub struct FrameworkInfo {
    pub charge_percentage: Option<u32>,
    pub charger_voltage: Option<u32>,
    pub charger_current: Option<u32>,
    pub design_capacity: Option<u32>,
    pub last_full_charge_capacity: Option<u32>,
    pub cycle_count: Option<u32>,
    pub capacity_loss_percentage: Option<f32>,
    pub capacity_loss_per_cycle: Option<f32>,
    pub is_charging: bool,
    pub is_ac_connected: bool,
    pub charging_status: &'static str,
    pub max_charge_limit: Option<u8>,
    pub is_microphone_enabled: bool,
    pub is_camera_enabled: bool,
    pub fp_brightness_percentage: Option<u8>,
    // pub fp_brightness_level: Option<FpLedBrightnessLevel>,
    pub kb_brightness_percentage: Option<u8>,
    pub smbios_version: Option<String>,
    pub smbios_release_date: Option<String>,
    pub smbios_vendor: Option<String>,
    pub pd_ports: Vec<Option<framework_lib::power::UsbPdPowerInfo>>,
}

impl FrameworkInfo {
    pub fn new(
        power: &Option<PowerInfo>,
        charge_limit: &Option<(u8, u8)>,
        privacy: &Option<(bool, bool)>,
        fp_brightness: &Option<(u8, Option<FpLedBrightnessLevel>)>,
        kb_brightness: Option<u8>,
        smbios: &Option<SMBiosData>,
        pd_ports: Vec<Option<UsbPdPowerInfo>>,
    ) -> Self {
        Self {
            charge_percentage: charge_percentage(power),
            charger_voltage: charger_voltage(power),
            charger_current: charger_current(power),
            design_capacity: design_capacity(power),
            last_full_charge_capacity: last_full_charge_capacity(power),
            cycle_count: cycle_count(power),
            capacity_loss_percentage: capacity_loss_percentage(power),
            capacity_loss_per_cycle: capacity_loss_per_cycle(power),
            is_charging: is_charging(power),
            is_ac_connected: is_ac_connected(power),
            charging_status: charging_status(power),
            max_charge_limit: max_charge_limit(charge_limit),
            is_microphone_enabled: is_microphone_enabled(privacy),
            is_camera_enabled: is_camera_enabled(privacy),
            fp_brightness_percentage: fp_brightness_percentage(fp_brightness),
            // fp_brightness_level: fp_brightness_level(fp_brightness),
            kb_brightness_percentage: kb_brightness_percentage(kb_brightness),
            smbios_version: smbios_version(smbios),
            smbios_release_date: smbios_release_date(smbios),
            smbios_vendor: smbios_vendor(smbios),
            pd_ports,
        }
    }
}

fn charge_percentage(power: &Option<PowerInfo>) -> Option<u32> {
    power.as_ref().and_then(|power| {
        power
            .battery
            .as_ref()
            .map(|battery| battery.charge_percentage)
    })
}

fn charger_voltage(power: &Option<PowerInfo>) -> Option<u32> {
    power.as_ref().and_then(|power| {
        power
            .battery
            .as_ref()
            .map(|battery| battery.present_voltage)
    })
}

fn charger_current(power: &Option<PowerInfo>) -> Option<u32> {
    power
        .as_ref()
        .and_then(|power| power.battery.as_ref().map(|battery| battery.present_rate))
}

fn design_capacity(power: &Option<PowerInfo>) -> Option<u32> {
    power.as_ref().and_then(|power| {
        power
            .battery
            .as_ref()
            .map(|battery| battery.design_capacity)
    })
}

fn last_full_charge_capacity(power: &Option<PowerInfo>) -> Option<u32> {
    power.as_ref().and_then(|power| {
        power
            .battery
            .as_ref()
            .map(|battery| battery.last_full_charge_capacity)
    })
}

fn cycle_count(power: &Option<PowerInfo>) -> Option<u32> {
    power
        .as_ref()
        .and_then(|power| power.battery.as_ref().map(|battery| battery.cycle_count))
}

fn is_charging(power: &Option<PowerInfo>) -> bool {
    power
        .as_ref()
        .and_then(|power| power.battery.as_ref().map(|battery| battery.charging))
        .unwrap_or(false)
}

fn is_ac_connected(power: &Option<PowerInfo>) -> bool {
    power
        .as_ref()
        .map(|power| power.ac_present)
        .unwrap_or(false)
}

fn capacity_loss_percentage(power: &Option<PowerInfo>) -> Option<f32> {
    match (design_capacity(power), last_full_charge_capacity(power)) {
        (Some(design), Some(last)) => Some(((design as f32 - last as f32) / design as f32) * 100.0),
        _ => None,
    }
}

fn capacity_loss_per_cycle(power: &Option<PowerInfo>) -> Option<f32> {
    match (capacity_loss_percentage(power), cycle_count(power)) {
        (Some(loss_percentage), Some(cycle_count)) => Some(loss_percentage / (cycle_count as f32)),
        _ => None,
    }
}

fn charging_status(power: &Option<PowerInfo>) -> &'static str {
    match (is_charging(power), is_ac_connected(power)) {
        (true, true) => "Charging",
        (false, true) => "Fully charged",
        (false, false) => "Discharging",
        (true, false) => "???",
    }
}

fn max_charge_limit(charge_limit: &Option<(u8, u8)>) -> Option<u8> {
    charge_limit.as_ref().map(|charge_limit| charge_limit.1)
}

fn is_microphone_enabled(privacy: &Option<(bool, bool)>) -> bool {
    privacy.as_ref().map(|privacy| privacy.0).unwrap_or(false)
}

fn is_camera_enabled(privacy: &Option<(bool, bool)>) -> bool {
    privacy.as_ref().map(|privacy| privacy.1).unwrap_or(false)
}

fn fp_brightness_percentage(
    fp_brightness: &Option<(u8, Option<FpLedBrightnessLevel>)>,
) -> Option<u8> {
    fp_brightness.as_ref().map(|fp_brightness| fp_brightness.0)
}

// fn fp_brightness_level(
//     fp_brightness: &Option<(u8, Option<FpLedBrightnessLevel>)>,
// ) -> Option<FpLedBrightnessLevel> {
//     fp_brightness
//         .as_ref()
//         .and_then(|fp_brightness| fp_brightness.1 )
// }

fn kb_brightness_percentage(kb_brightness: Option<u8>) -> Option<u8> {
    kb_brightness
}

fn smbios_version(smbios: &Option<SMBiosData>) -> Option<String> {
    smbios.as_ref().and_then(|smbios| {
        smbios
            .iter()
            .find_map(|undefined_struct| match undefined_struct.defined_struct() {
                DefinedStruct::Information(data) => {
                    Some(smbios::dmidecode_string_val(&data.version()))
                }
                _ => None,
            })
            .flatten()
    })
}

fn smbios_release_date(smbios: &Option<SMBiosData>) -> Option<String> {
    smbios.as_ref().and_then(|smbios| {
        smbios
            .iter()
            .find_map(|undefined_struct| match undefined_struct.defined_struct() {
                DefinedStruct::Information(data) => {
                    Some(smbios::dmidecode_string_val(&data.release_date()))
                }
                _ => None,
            })
            .flatten()
    })
}

fn smbios_vendor(smbios: &Option<SMBiosData>) -> Option<String> {
    smbios.as_ref().and_then(|smbios| {
        smbios
            .iter()
            .find_map(|undefined_struct| match undefined_struct.defined_struct() {
                DefinedStruct::Information(data) => {
                    Some(smbios::dmidecode_string_val(&data.vendor()))
                }
                _ => None,
            })
            .flatten()
    })
}
