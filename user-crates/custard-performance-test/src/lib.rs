use std::time::SystemTime;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BenchmarkTask {
	counter: u64,
	#[serde(default = "SystemTime::now")]
	time: SystemTime,
}
