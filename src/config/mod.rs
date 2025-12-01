use serde_json::from_str;

use crate::util::str::unwrap_string;

pub struct DatabaseConfig {
	pub host: &'static str,
	pub port: i64,
	pub db_name: &'static str,
	pub username: Option<&'static str>,
	pub password: Option<&'static str>
}

pub struct Config {
	pub host: &'static str,
	pub port: i64,
	pub database: DatabaseConfig
}

impl Config {
	pub fn new() -> Self {
		let mut host = "127.0.0.1";
		let host_raw = std::env::var("HOST");
		match host_raw {
			Ok(h) => {
				host = unwrap_string(h);
			},
			Err(_) => {
				println!("Failed to load host value from .env");
			}
		}

		let mut port: i64 = 3000;
		let port_raw = std::env::var("PORT");
		match port_raw {
			Ok(p) => {
				port = from_str::<i64>(unwrap_string(p)).unwrap();
			},
			Err(_) => {
				println!("Failed to load port value from .env");
			}
		}

		let mut db_host = "127.0.0.1";
		let db_host_raw = std::env::var("DB_HOST");
		match db_host_raw {
			Ok(h) => {
				db_host = unwrap_string(h);
			},
			Err(_) => {
				println!("Failed to load database host value from .env");
			}
		}

		let mut db_port = 5432;
		let db_port_raw = std::env::var("DB_PORT");
		match db_port_raw {
			Ok(p) => {
				db_port = from_str::<i64>(unwrap_string(p)).unwrap();
			},
			Err(_) => {
				println!("Failed to load database port value from .env");
			}
		}

		let mut db_name = "boilerplate";
		let db_name_raw = std::env::var("DB_NAME");
		match db_name_raw {
			Ok(n) => {
				db_name = unwrap_string(n);
			},
			Err(_) => {
				println!("Failed to load database name value from .env");
			}
		}

		let mut username: Option<&str> = None;
		let username_raw = std::env::var("DB_USERNAME");
		if let Ok(v) = username_raw {
			username = Some(unwrap_string(v));
		}

		let mut password: Option<&str> = None;
		let password_raw = std::env::var("DB_PASSWORD");
		if let Ok(v) = password_raw {
			password = Some(unwrap_string(v));
		}

		let database: DatabaseConfig = DatabaseConfig {
			host: db_host,
			port: db_port,
			db_name,
			username,
			password
		}; 

		return Self {
			host,
			port,
			database
		}
	}
}
