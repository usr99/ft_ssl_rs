use std::io::{Read, Write};

use super::{Cipher, Kind};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
	/// decode mode
	#[arg(short)]
	decode: bool,
	
	/// encode mode
	#[arg(short, default_value = "true")]
	encode: bool,
	
	/// input file
	#[arg(short)]
	input_file: Option<String>,
	
	/// output file
	#[arg(short)]
	output_file: Option<String>
}

impl Cipher for Options {
	fn execute(&self) -> anyhow::Result<()> {
		let mut input = Vec::new();
		match self.input_file {
			Some(ref filename) => std::fs::File::open(filename)?.read_to_end(&mut input)?,
			None => std::io::stdin().read_to_end(&mut input)?
		};

		let result = match self.decode {
			true => ft_ssl::base64::decode(&input),
			false => ft_ssl::base64::encode(&input)
		};

		match self.output_file {
			Some(ref filename) => std::fs::File::options().write(true).create(true).open(filename)?.write_all(&result)?,
			None => {
				std::io::stdout().write_all(&result)?;
				print!("\n");
			}
		};

		Ok(())
	}
}
