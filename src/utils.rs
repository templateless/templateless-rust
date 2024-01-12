use dotenvy::dotenv;
use std::process;

pub fn get_env(key: &str) -> String {
	dotenv().ok();

	match dotenvy::var(key) {
		Ok(value) if !value.is_empty() => value,
		_ => {
			println!("Missing environment variable {key}");
			process::exit(exitcode::CONFIG);
		}
	}
}
