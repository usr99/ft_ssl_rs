use super::Cipher;
use ft_ssl::{hex::ToHexString, md5, sha256};

use anyhow::Result;
use clap::{Args, Parser, ValueEnum};
use std::{env::ArgsOs, fmt::Display, io::Read};

#[derive(Debug, Args, Clone)]
struct Options {
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
	files: Vec<String>
}

enum Input<'a> {
	Stdin { buf: Vec<u8>, echo: bool },
	File(&'a str),
	CommandLine(&'a str)
}

#[derive(Clone, Copy)]
enum Output {
	Default,
	Reversed,
	Quiet
}

#[derive(Debug, Parser, Clone)]
pub struct MD5Options {
	#[clap(flatten)]
	opts: Options
}

impl Cipher for MD5Options {
	fn execute(&self) -> Result<()> { inner_execute("MD5", md5::hash, &self.opts) }
}

#[derive(Debug, Parser, Clone)]
pub struct SHA256Options {
	#[clap(flatten)]
	opts: Options
}

impl Cipher for SHA256Options {
	fn execute(&self) -> Result<()> { inner_execute("SHA256", sha256::hash, &self.opts) }
}

fn inner_execute(name: &str, algorithm: fn(&[u8]) -> Vec<u8>, opts: &Options) -> Result<()> {
	let mut output_mode = Output::Default;
	if opts.reverse {
		output_mode = Output::Reversed;
	}
	// quiet mode overrides reversed mode
	if opts.quiet {
		output_mode = Output::Quiet;
	}

	if opts.echo_stdin || (opts.files.is_empty() && opts.strings.is_empty()) {
		let mut buf = Vec::new();
		std::io::stdin().read_to_end(&mut buf)?;

		let hash = algorithm(&buf);
		print_formatted_hash(
			name,
			&hash,
			Input::Stdin {
				buf,
				echo: opts.echo_stdin
			},
			output_mode
		);
	}

	for str in opts.strings.iter() {
		let hash = algorithm(str.as_bytes());
		print_formatted_hash(name, &hash, Input::CommandLine(str), output_mode);
	}

	for filename in opts.files.iter() {
		let mut buf = Vec::new();
		let mut file = match std::fs::File::open(&filename) {
			Ok(file) => file,
			Err(e) => {
				eprintln!("{filename}: {e}");
				continue;
			}
		};
		file.read_to_end(&mut buf)?;

		let hash = algorithm(&buf);
		print_formatted_hash(name, &hash, Input::File(filename), output_mode);
	}

	Ok(())
}

fn print_formatted_hash(name: &str, hash: &[u8], src: Input, output_mode: Output) {
	// Convert u8 slice into a hex string representation
	let hash = hash.encode_hex();

	match src {
		Input::Stdin { buf, echo } => {
			if echo {
				// of course 'buf' might contain non valid utf-8
				// but for this project I need to recreate any bug that the previous C implementation had
				// "it’s not a bug, it’s a feature"
				let message = unsafe { std::str::from_utf8_unchecked(&buf) }.trim_end_matches('\n');
				if let Output::Quiet = output_mode {
					println!("{message}");
				} else {
					print!("(\"{message}\")= ");
				}
			} else {
				if let Output::Default | Output::Reversed = output_mode {
					print!("(stdin)= ");
				}
			}
			println!("{hash}");
		}
		Input::File(ref message) | Input::CommandLine(ref message) => {
			let mut quotes = "";
			if let Input::CommandLine(_) = src {
				quotes = "\"";
			}

			match output_mode {
				Output::Default => println!("{name} ({quotes}{message}{quotes}) = {hash}"),
				Output::Reversed => println!("{hash} {quotes}{message}{quotes}"),
				Output::Quiet => println!("{hash}")
			}
		}
	}
}
