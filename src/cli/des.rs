use super::Cipher;
use super::CipheringArg;

use anyhow::Result;
use clap::Parser;
use std::{
	fs::File,
	io::{self, Read, Write}
};

#[derive(Debug, Parser)]
pub struct Options {
	/// decode/encode the input/output in base64
	#[arg(short = 'a', long)]
	base64: bool,

	#[clap(flatten)]
	ciphering: CipheringArg,

	/// input file
	#[arg(short)]
	infile: Option<String>,

	/// hex key
	#[arg(short)]
	key: Option<String>,

	/// output file
	#[arg(short)]
	outfile: Option<String>,

	/// ascii password
	#[arg(short)]
	password: Option<String>,

	/// hex salt
	#[arg(short)]
	salt: Option<String>
}

#[derive(Debug, Parser)]
pub struct ECBOptions {
	#[clap(flatten)]
	opts: Options
}

impl Cipher for ECBOptions {
	fn execute(&self) -> Result<()> {
		// has key ?
		// no
		// has salt ?
		// no -> gen salt
		// key = PBKDF

		// has input file ?
		// yes -> read input file
		// no -> read stdin
		// is base64 + decode ?
		// decode input
		dbg!(&self);

		let mut input = Vec::new();
		match self.opts.infile {
			Some(ref filename) => File::open(filename)?.read_to_end(&mut input)?,
			None => io::stdin().read_to_end(&mut input)?
		};

		// if (self.opts.base64 && self.opts.decrypt) {
		// 	input = ft_ssl::base64::decode(&input);
		// }

		// des encryption
		let result = [0, 1, 2, 3, 4];

		// if (self.opts.base64 &&

		match self.opts.outfile {
			Some(ref filename) => File::options()
				.write(true)
				.create(true)
				.open(filename)?
				.write_all(&result)?,
			None => {
				io::stdout().write_all(&result)?;
				print!("\n");
			}
		};

		Ok(())
	}
}

#[derive(Debug, Parser)]
pub struct CBCOptions {
	#[clap(flatten)]
	opts: Options,

	/// hex initialization vector
	#[arg(short = 'v')]
	iv: Option<String>
}

impl Cipher for CBCOptions {
	fn execute(&self) -> anyhow::Result<()> { Ok(()) }
}
