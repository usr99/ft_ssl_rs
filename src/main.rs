#![allow(unused)]
use std::{collections::HashMap, fmt::format, io::Read};

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

#[repr(usize)]
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Hash)]
#[clap(rename_all = "snake_case")]
enum Encryption {
	MD5,
	SHA256
}

const ENCRYPTION_METHODS: [Hasher; 2] = [
	md5::hash,
	sha256::hash
];

fn main() -> Result<()> {
	let cli = CLI::parse();
	let encrypt = ENCRYPTION_METHODS.get(cli.command as usize).unwrap();

	if cli.echo_stdin || (cli.file.is_empty() && cli.strings.is_empty()) {
		let mut buf = Vec::new();
		std::io::stdin().read_to_end(&mut buf)?;

		println!("(stdin) = {}", encrypt(&buf));
	}

	for str in cli.strings {
		println!("{} = {}", str, encrypt(str.as_bytes()));
	}

	for filename in cli.file {
		let mut buf = Vec::new();
		let mut file = std::fs::File::open(&filename)?;
		file.read_to_end(&mut buf);

		println!("({}) = {}", filename, encrypt(&buf));
	}

	Ok(())
}
