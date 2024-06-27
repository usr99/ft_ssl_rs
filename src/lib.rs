#![allow(unused)]

pub mod base64;
pub mod hex;
pub mod des;

#[macro_export]
macro_rules! align {
	($value:expr, $lim:expr) => { (($value + $lim - 1) & !($lim - 1)) }
}

pub trait Hash {
	const BLOCK_SIZE: usize;
	const DIGEST_SIZE: usize;

	type Digest: AsRef<[u8]> + AsMut<[u8]>;

	fn hash(message: &[u8]) -> Self::Digest;
}

mod md5;
pub use md5::MD5;

mod sha256;
pub use sha256::SHA256;

trait PRF {
	type Output: AsRef<[u8]>;

	fn randgen(key: &[u8], message: &[u8]) -> Self::Output;
}

mod hmac;
pub use hmac::HMAC;

mod pbkdf2;

