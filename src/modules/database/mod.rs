use std::{fs, ptr};
use std::cell::UnsafeCell;

use tokio::runtime::{Runtime};
use sqlx::{Row, Pool, Postgres, postgres::PgPoolOptions};

use crate::{config::Config, declare_module, module_init, modules::ServiceModule, util::str::unwrap_string};

pub struct DatabaseModule {
	pool: UnsafeCell<*mut Pool<Postgres>>
}

declare_module!(DatabaseModule);

fn build_credential(u: &str, p: &str) -> &'static str {
	let password = unwrap_string(urlencoding::encode(p).to_string());
	return unwrap_string(format!("{}:{}", u, password));
}

fn build_uri(c: &str, host: &str, port: i64, db_name: &str) -> &'static str {
	if c == "" {
		return unwrap_string(format!("postgresql://{}:{}/{}", host, port, db_name));
	}

	return unwrap_string(format!("postgresql://{}@{}:{}/{}", c, host, port, db_name))
}

async fn migrate(pool: Pool<Postgres>) {
	let tbl = sqlx::query("CREATE TABLE IF NOT EXISTS migrations (
		version 	VARCHAR(36),
		applied_at	TIMESTAMPTZ	DEFAULT NOW()
	);".trim())
	.execute(&pool)
	.await;

	if let Err(e) = tbl {
		println!("Failed create migration table: {:?}", e);
		return;
	}

	let applied: Vec<String> = sqlx::query("SELECT version FROM migrations")
		.fetch_all(&pool)
		.await
		.expect("Failed to load migrations list")
		.iter()
		.map(|r| r.get("version"))
		.collect();

	let mut files: Vec<_> = fs::read_dir("./migrations")
		.unwrap()
		.filter_map(|e| e.ok())
		.filter(|e| e.path().extension().map(|ext| ext == "sql").unwrap_or(false))
		.collect();

	files.sort_by_key(|e| e.file_name());

	for entry in files {
		let name = entry.file_name().to_string_lossy().to_string();
		if applied.contains(&name) {
			continue;
		}

		let content = fs::read_to_string(entry.path());
		match content {
			Ok(c) => {
				sqlx::query(&c).execute(&pool)
					.await
					.expect(unwrap_string(format!("Failed to running migration: {}", name)));
				sqlx::query("INSERT INTO migrations (name) VALUES ($1)")
					.bind(&name)
					.execute(&pool)
					.await
					.expect("Failed to update migration value");
			},
			Err(_) => {
				continue;
			}
		};
	}

	println!("{:?}", applied);
}

async fn connect() -> Pool<Postgres> {
	let config = Config::new();
	let db = config.database;
	let mut credential = "";
	if !db.username.is_none() && !db.password.is_none() {
		credential = build_credential(db.username.unwrap(), db.password.unwrap());
	}

	let uri = build_uri(credential, db.host, db.port, db.db_name);

	let p = PgPoolOptions::new()
		.max_connections(10)
		.connect(uri)
		.await
		.expect("Failed to connect database");

	let res = sqlx::query("SELECT 1")
		.execute(&p)
		.await;

	if let Err(e) = res {
		println!("{:?}", e);
	}

	migrate(p.clone()).await;
	return p;
}

async fn disconnect(pool: Pool<Postgres>) {
	pool.close().await;
}

impl DatabaseModule {
	fn new() -> Self {
		println!("Database: module loaded");

		return Self {
			pool: UnsafeCell::new(ptr::null_mut())
		};
	}

	fn get_pool(&self) -> *mut Pool<Postgres> {
		unsafe {
			return *self.pool.get();
		}
	}
}

impl ServiceModule for DatabaseModule {
	fn name(&self) -> &'static str { "database" }

	fn init(&self) {
		let pool: Pool<Postgres> = std::thread::spawn(|| {
			let rt = Runtime::new().unwrap();
			rt.block_on(connect())
		})
		.join()
		.expect("Failed to create database pool");

		unsafe {
			let db_ptr = Box::into_raw(Box::new(pool));
			*self.pool.get() = db_ptr;
		}
	}

	fn destroy(&self) {
		unsafe {
			let db_ptr = *self.pool.get();
			if db_ptr.is_null() {
				return;
			}

			let pool = Box::from_raw(db_ptr);
			std::thread::spawn(move || {
				let rt = Runtime::new().unwrap();
				rt.block_on(async {
					disconnect(*pool).await;
				});
			}).join().expect("Failed to unload database module");
		
			drop(Box::from_raw(db_ptr));
			*self.pool.get() = ptr::null_mut();
		}
	}
}

module_init!(__init);
