#![allow(unused)]
use std::fmt::Display;

pub mod md5;
pub mod sha256;
pub mod base64;

pub struct Hash(Vec<u8>);

impl Display for Hash {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.iter().fold(Ok(()), |result, byte| {
			result.and_then(|_| write!(f, "{:02x}", byte))
		})
	}
}

#[macro_export]
macro_rules! align {
	($value:expr, $lim:expr) => { (($value + $lim - 1) & !($lim - 1)) }
}

#[cfg(test)]
#[macro_export]
macro_rules! hash {
	($input:expr) => { format!("{}", super::hash($input.as_bytes())) }
}
