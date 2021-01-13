use std::time::{SystemTime, UNIX_EPOCH};

pub enum DurationType {
	Seconds,
	Milliseconds,
	Microseconds,
	Nanoseconds,
}
pub struct Timer {
	init_time: std::time::Duration,
}
impl Timer {
	pub fn new() -> Timer {
		Timer {
			init_time:  SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards"),
		}
	}
	pub fn get_time(&self, duration_type: DurationType) -> u128 {
		let now = SystemTime::now();
		let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

		match duration_type {
			DurationType::Seconds => return (since_the_epoch.as_secs() - self.init_time.as_secs()).into(),
			DurationType::Milliseconds => return (since_the_epoch.as_millis() - self.init_time.as_millis()).into(),
			DurationType::Microseconds => return (since_the_epoch.as_micros() - self.init_time.as_micros()).into(),
			DurationType::Nanoseconds => return (since_the_epoch.as_nanos() - self.init_time.as_nanos()).into(),
		}
	}
}