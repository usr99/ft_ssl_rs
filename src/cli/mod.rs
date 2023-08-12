mod hash;
mod base64;
mod des;

use std::{
	env::ArgsOs,
	ffi::OsString,
	iter::{Once, Chain}
};
use clap::Parser;
use anyhow::{Result, anyhow};

pub trait Cipher {
	fn execute(&self) -> anyhow::Result<()>;
}

type Iter = Chain<Once<OsString>, ArgsOs>;

macro_rules! parse_options {
	($struct_name:ty) => {
		|args: Iter| Box::new(<$struct_name>::parse_from(args))
	}
}

const ENCRYPTIONS: [(&str, fn(Iter) -> Box<dyn Cipher>); 2] = [
	("md5", parse_options!(hash::MD5Options)),
	("sha256", parse_options!(hash::SHA256Options))
];

pub fn parse_cipher(command: &str, mut args: ArgsOs) -> Option<Box<dyn Cipher>> {
	match ENCRYPTIONS.iter().find(|(name, _)| &command == name) {
		Some((_, parse)) => {
			let mut prefix = args.next().unwrap();
			prefix.push(" ");
			prefix.push(args.next().unwrap());
			let args = std::iter::once(prefix).chain(args);

			Some(parse(args))
		},
		None => None
	}
}

pub fn get_commands_list() -> String {
	let mut list = String::new();
	
	for (name, _) in ENCRYPTIONS.iter() {
		list.push_str(name);
		list.push('\n');
	}

	return list;
}

enum Kind {
	Encode,
	Decode
}
