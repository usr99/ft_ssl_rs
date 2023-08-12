#![allow(unused)]
mod cli;

use clap::Parser;
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
	let mut args = std::env::args();
	let program_name = args.next().unwrap();
	let command = match args.next() {
		Some(cmd) => cmd,
		None => return Err(format_usage_error(&format!("usage: {program_name} <COMMAND>")))
	};

	match cli::parse_cipher(&command, std::env::args_os()) {
		Some(cipher) => cipher.execute()?,
		None => return Err(format_usage_error(&format!("{program_name}: bad command: {command}")))
	};

	Ok(())
}

fn format_usage_error(message: &str) -> anyhow::Error {
	anyhow!(format!("{message}\n\n{}", cli::get_commands_list()))
}
