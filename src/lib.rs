#![allow(unused)]

pub mod base64;
pub mod des;

pub mod md5;
pub mod sha256;
pub mod hmac;

#[macro_export]
macro_rules! align {
	($value:expr, $lim:expr) => { (($value + $lim - 1) & !($lim - 1)) }
}

pub mod hex {
	use itertools::Itertools;

	const HEX_BASE: &'static str = "0123456789abcdef";

	pub trait ToHexString {
		fn encode_hex(&self) -> String;
	}

	impl<T: AsRef<[u8]>> ToHexString for T {
		fn encode_hex(&self) -> String {
			let bytes = self.as_ref();
			let mut hex = String::with_capacity(bytes.len() * 2);

			let base = HEX_BASE.as_bytes();
			for byte in bytes {
				hex.push(base[*byte as usize >> 4] as char);
				hex.push(base[*byte as usize & 0xf] as char);
			}

			hex
		}
	}
	
	pub trait FromHexString {
		fn decode_hex(&self) -> Option<Vec<u8>>;
	}

	impl FromHexString for &str {
		fn decode_hex(&self) -> Option<Vec<u8>> {
			// A valid hex string contains only hex characters and has an even length
			// checking this allows us to unwrap safely later
			if !self.chars().all(|c| HEX_BASE.contains(c)) || self.len() & 1 != 0 {
				return None;
			}

			// Divide the hex string into chunks of 2 digits
			// then for each pair compute the numeric value
			let bytes = self.chars().chunks(2).into_iter().map(|hexpair| {
				let mut chars = hexpair.into_iter();
				let hi = HEX_BASE.find(chars.next().unwrap()).unwrap();
				let lo = HEX_BASE.find(chars.next().unwrap()).unwrap();

				(hi << 4 | lo) as u8
			}).collect();

			Some(bytes)
		}
	}
}

