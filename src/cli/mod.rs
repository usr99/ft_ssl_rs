mod hash;
mod base64;
mod des;

use std::{
	env::ArgsOs,
	ffi::OsString,
	iter::{Once, Chain}
};
use clap::{Parser, Command, Arg};
use anyhow::anyhow;

pub trait Cipher {
	fn execute(&self) -> anyhow::Result<()>;
}

macro_rules! parse_options {
	($struct_name:ty) => {
		|args: Vec<String>| {
			<$struct_name>::try_parse_from(args)
				.and_then(|opt| Ok(Box::new(opt) as Box<dyn Cipher>))
		}
	}
}

const ENCRYPTIONS: [(&str, fn(Vec<String>) -> Result<Box<dyn Cipher>, clap::error::Error>); 2] = [
	("md5", parse_options!(hash::MD5Options)),
	("sha256", parse_options!(hash::SHA256Options))
];

pub fn parse_cipher(command: &str, args: Vec<String>) -> Option<Box<dyn Cipher>> {
	match ENCRYPTIONS.iter().find(|(name, _)| &command == name) {
		Some((_, parse)) => match parse(args) {
			Ok(cipher) => Some(cipher),
			Err(e) => { e.print(); None }
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

enum Kind {
	Encode,
	Decode
}
