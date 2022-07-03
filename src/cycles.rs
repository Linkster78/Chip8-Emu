use std::cmp::max;
use std::time::SystemTime;

pub struct Coordinator {
    pub rate: u32,
    offset: u128,
    last_execution: u128
}

impl Coordinator {
    pub fn new(rate: u32) -> Self {
        Coordinator {
            rate,
            offset: 1_000_000_000 / rate as u128,
            last_execution: 0
        }
    }

    pub fn should_cycle(&mut self) -> bool {
        let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos();
        if timestamp_nanos - self.last_execution >= self.offset {
            self.last_execution = timestamp_nanos;
            true
        } else {
            false
        }
    }

    pub fn delay_until_cycle(&self) -> u128 {
        let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos();
        max((self.last_execution + self.offset) as i128 - timestamp_nanos as i128, 0) as u128
    }

    pub fn smallest_delay_until_cycle(coordinators: &[&Coordinator]) -> u128 {
        coordinators.iter().map(|coordinator| coordinator.delay_until_cycle())
            .min().unwrap()
    }
}