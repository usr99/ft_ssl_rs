#![allow(unused)]
use std::{collections::HashMap, fmt::{format, Display}, io::Read};

use ft_ssl::*;
use clap::{Parser, ValueEnum};
use anyhow::Result;

#[derive(Parser, Debug)]
struct CLI {
	/// encryption mode
	command: Encryption,

	/// echo STDIN to STDOUT
	#[arg(short = 'p')]
	echo_stdin: bool,
	
	/// quiet mode
	#[arg(short)]
	quiet: bool,
	
	/// reverse the format of the output
	#[arg(short)]
	reverse: bool,
	
	/// additionnal string input
	#[arg(short)]
	strings: Vec<String>,

	/// file input
	file: Vec<String>,
}

enum Input {
	Stdin {
		buf: Vec<u8>,
		echo: bool
	},
	File(String),
	CommandLine(String)
}

enum Output {
	Default,
	Reversed,
	Quiet
}

pub struct Configuration {
	encryption: Encryption,
	algorithm: Hasher,
	output_mode: Output
}

#[repr(usize)]
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[clap(rename_all = "snake_case")]
enum Encryption {
	MD5,
	SHA256
}

impl Display for Encryption {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// relies on the derived Debug implementation
		write!(f, "{:?}", self)
	}
}

fn main() -> Result<()> {
	let cli = CLI::parse();
	let config = define_configuration(&cli);

	if cli.echo_stdin || (cli.file.is_empty() && cli.strings.is_empty()) {
		let mut buf = Vec::new();
		std::io::stdin().read_to_end(&mut buf)?;

		let hash = (config.algorithm)(&buf);
		print_formatted_hash(hash, Input::Stdin { buf, echo: cli.echo_stdin }, &config);
	}

	for str in cli.strings {
		let hash = (config.algorithm)(str.as_bytes());
		print_formatted_hash(hash, Input::CommandLine(str), &config);
	}

	for filename in cli.file {
		let mut buf = Vec::new();
		let mut file = std::fs::File::open(&filename)?;
		file.read_to_end(&mut buf);

		let hash = (config.algorithm)(&buf);
		print_formatted_hash(hash, Input::File(filename), &config);
	}

	Ok(())
}

fn define_configuration(parameters: &CLI) -> Configuration {
	const ENCRYPTION_METHODS: [Hasher; 2] = [
		md5::hash,
		sha256::hash
	];

	let mut output_mode = Output::Default;
	if parameters.reverse {
		output_mode = Output::Reversed;
	}
	// quiet mode overrides reversed mode
	if parameters.quiet {
		output_mode = Output::Quiet;
	}
	
	let algorithm_idx = parameters.command as usize;
	Configuration {
		encryption: parameters.command,
		algorithm: ENCRYPTION_METHODS[algorithm_idx],
		output_mode
	}
}

fn print_formatted_hash(hash: Hash, src: Input, config: &Configuration) {
	match src {
		Input::Stdin { buf, echo } => {
			if echo {
				let message = unsafe { std::str::from_utf8_unchecked(&buf) }.trim_end_matches('\n');
				if let Output::Quiet = config.output_mode {
					println!("{message}");
				} else {
					print!("(\"{message}\")= ");
				}
			} else {
				if let Output::Default | Output::Reversed = config.output_mode {
					print!("(stdin)= ");
				}
			}
			println!("{hash}");
		},
		Input::File(ref message) | Input::CommandLine(ref message) => {
			let mut quotes = "";
			if let Input::CommandLine(_) = src {
				quotes = "\"";
			}
	
			let algname = config.encryption;
			match config.output_mode {
				Output::Default => println!("{algname} ({quotes}{message}{quotes}) = {hash}"),
				Output::Reversed => println!("{hash} {quotes}{message}{quotes}"),
				Output::Quiet => println!("{hash}"),
			}			
		}
	}
}
