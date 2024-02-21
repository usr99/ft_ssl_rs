mod base64;
mod des;
mod hash;

use anyhow::anyhow;
use clap::{Command, Parser, ArgAction::SetTrue, ValueEnum};
use std::{
	env::ArgsOs,
	ffi::OsString,
	iter::{Chain, Once}
};

pub trait Cipher {
	fn execute(&self) -> anyhow::Result<()>;
}

macro_rules! parse_options {
	($struct_name:ty) => {
		|args: Vec<String>| {
			<$struct_name>::try_parse_from(args)
				.and_then(|opt| Ok(Box::new(opt) as Box<dyn Cipher>))
		}
	};
}

const ENCRYPTIONS: [(
	&str,
	fn(Vec<String>) -> Result<Box<dyn Cipher>, clap::error::Error>
); 6] = [
	("md5", parse_options!(hash::MD5Options)),
	("sha256", parse_options!(hash::SHA256Options)),
	("base64", parse_options!(base64::Options)),
	("des", parse_options!(des::ECBOptions)),
	("des-ecb", parse_options!(des::ECBOptions)),
	("des-cbc", parse_options!(des::CBCOptions))
];

pub fn parse_cipher(command: &str, args: Vec<String>) -> Option<Box<dyn Cipher>> {
	match ENCRYPTIONS.iter().find(|(name, _)| &command == name) {
		Some((_, parse)) => match parse(args) {
			Ok(cipher) => Some(cipher),
			Err(e) => {
				e.print();
				None
			}
		},
		None => {
			eprintln!("bad command: {command}\n");
			for (name, _) in ENCRYPTIONS {
				eprintln!("{name}")
			}
			eprintln!("");

			None
		}
	}
}

#[derive(ValueEnum, Copy, Clone, Debug)]
enum Ciphering {
	Encrypt,
	Decrypt
}

#[derive(clap::Args, Copy, Clone, Debug)]
#[group(multiple = false)]
struct CipheringArg {
	/// encrypt (default)
	#[arg(short='e', overrides_with = "decrypt")]
	encrypt: bool,

	/// decrypt
	#[arg(short='d', overrides_with = "encrypt")]
	decrypt: bool,

	#[arg(hide = true, default_value = "encrypt", default_value_ifs=[
		("encrypt", "true", "encrypt"),
		("decrypt", "true", "decrypt")
	])]
	kind: Ciphering
}
