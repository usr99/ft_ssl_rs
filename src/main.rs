#![allow(unused)]
mod cli;

use std::io::Write;
use clap::Parser;
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
	let mut args = std::env::args().collect::<Vec<_>>();

	if args.len() > 1 {
		let program = args[0].clone();

		// We already parsed the command
		// but we still want clap to print the program name
		// so we merge the first two arguments into one
		let command = args.remove(1);
		args[0].push(' ');
		args[0].push_str(&command);

		if let Some(cipher) = cli::parse_cipher(&command, args) {
			cipher.execute()?;
		}
	} else {
		let mut buf = String::new();

		while let Ok(bytes) = {
			print!("ft_ssl > ");
			std::io::stdout().flush();
			std::io::stdin().read_line(&mut buf)
		} {
			if bytes == 0 {
				break ;
			}
			
			let mut args = buf.split_whitespace().map(|str| str.to_owned()).collect::<Vec<_>>();
			if args.len() == 0 {
				continue ;
			}

			let command = args[0].clone();
			if let Some(cipher) = cli::parse_cipher(&command, args) {
				cipher.execute()?;
			}

			buf.clear();
		}
	}

	Ok(())
}
