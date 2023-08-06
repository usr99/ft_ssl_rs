use std::fmt::Display;

pub mod md5;
pub mod sha256;

pub struct Hash(Vec<u8>);

impl Display for Hash {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.iter().fold(Ok(()), |result, byte| {
			result.and_then(|_| write!(f, "{:x}", byte))
		})
	}
}

#[cfg(test)]
#[macro_export]
macro_rules! hash {
	($input:expr) => { format!("{}", super::hash($input.as_bytes())) }
}