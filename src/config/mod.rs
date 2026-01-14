use std::env;

pub struct VersionInfo {
	pub name: String,
	pub version: String,
	pub branch: String,
	pub hash: String,
	pub build_time: String
}

pub struct Config {
	pub host: String,
	pub port: i32,
	pub info: VersionInfo
}

impl Config {
	pub fn new() -> Self {
		let host = env::var("HOST").expect("0.0.0.0");
		let raw_port = env::var("PORT").expect("3306");
		let port = match raw_port.parse::<i32>() {
			Ok(v) => v,
			Err(_) => 3000
		};

		let info = VersionInfo {
			name: option_env!("APP_NAME").unwrap_or("Sample App").to_string(),
			version: option_env!("APP_VERSION").unwrap_or("0.1.0").to_string(),
			branch: option_env!("APP_BRANCH").unwrap_or("unknown").to_string(),
			hash: option_env!("APP_HASH").unwrap_or("unknown").to_string(),
			build_time: option_env!("APP_BUILD_TIME").unwrap_or("unknown").to_string()
		};

		return Self { host, port, info };
	}
}
