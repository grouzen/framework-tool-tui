use framework_lib::chromium_ec::CrosEc;
use framework_lib::power::PowerInfo;
use std::time::Duration;
use std::time::Instant;

pub struct Framework {
    pub controls: FrameworkControls,
    ec: CrosEc,
    last_poll: Instant,
    poll_interval: Duration,
}

impl Framework {
    pub fn new(ec: CrosEc, poll_interval: Duration) -> Self {
        Framework {
            controls: Default::default(),
            ec,
            last_poll: Instant::now(),
            poll_interval,
        }
    }

    pub fn poll(&mut self) {
        let power = framework_lib::power::power_info(&self.ec);
        // let fan_count = framework_lib::power::get_fan_num(&self.ec).ok();
        // let pd_info = Some(
        //     framework_lib::power::get_pd_info(&self.ec, 4)
        //         .into_iter()
        //         .filter_map(Result::ok)
        //         .collect::<Vec<_>>(),
        // );

        self.controls = FrameworkControls {
            power,
            // fan_count,
            // pd_info,
        }
    }

    pub fn poll_if_needed(&mut self) {
        if self.last_poll.elapsed() >= self.poll_interval {
            self.poll();
            self.last_poll = Instant::now();
        }
    }
}

#[derive(Default)]
pub struct FrameworkControls {
    pub power: Option<PowerInfo>,
    // pub fan_count: Option<usize>,
    // pub pd_info: Option<Vec<UsbPdPowerInfo>>,
}

impl FrameworkControls {
    pub fn charge_percentage(&self) -> Option<u32> {
        self.power
            .as_ref()
            .map(|power| {
                power
                    .battery
                    .as_ref()
                    .map(|battery| battery.charge_percentage)
            })
            .flatten()
    }

    pub fn charger_voltage(&self) -> Option<u32> {
        self.power
            .as_ref()
            .map(|power| {
                power
                    .battery
                    .as_ref()
                    .map(|battery| battery.present_voltage)
            })
            .flatten()
    }

    pub fn charger_current(&self) -> Option<u32> {
        self.power
            .as_ref()
            .map(|power| power.battery.as_ref().map(|battery| battery.present_rate))
            .flatten()
    }

    pub fn design_capacity(&self) -> Option<u32> {
        self.power
            .as_ref()
            .map(|power| {
                power
                    .battery
                    .as_ref()
                    .map(|battery| battery.design_capacity)
            })
            .flatten()
    }

    pub fn last_full_charge_capacity(&self) -> Option<u32> {
        self.power
            .as_ref()
            .map(|power| {
                power
                    .battery
                    .as_ref()
                    .map(|battery| battery.last_full_charge_capacity)
            })
            .flatten()
    }

    pub fn cycle_count(&self) -> Option<u32> {
        self.power
            .as_ref()
            .map(|power| power.battery.as_ref().map(|battery| battery.cycle_count))
            .flatten()
    }

    pub fn is_charging(&self) -> bool {
        self.power
            .as_ref()
            .map(|power| power.battery.as_ref().map(|battery| battery.charging))
            .flatten()
            .unwrap_or(false)
    }

    pub fn is_ac_connected(&self) -> bool {
        self.power
            .as_ref()
            .map(|power| power.ac_present)
            .unwrap_or(false)
    }
}
