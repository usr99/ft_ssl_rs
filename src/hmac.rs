use super::hex::ToHexString;

const IPAD: u8 = 0x36;
const OPAD: u8 = 0x5c;

pub fn hmac(hash: fn(&[u8]) -> Vec<u8>, key: &[u8], message: &[u8], block_size: usize, output_size: usize) -> Vec<u8> {

	// let mut inner = vec![IPAD; block_size]; 
	// // xor
	// inner.extend_from_slice(message);
	// let inner = hash(&inner);
	//
	// let mut outer = vec![OPAD; block_size]; 
	// // xor key
	// outer.extend_from_slice(inner.as_ref());
	//
	// hash(&outer)

	vec![]
}

#[cfg(test)]
mod test {
	use ntest::test_case;
	use crate::sha256;

	#[test_case(name = "sha256_basic",
		"password",
		"The quick brown fox jumps over the lazy dog.",
		"6d0ed1a0c1a5e0f9fa971c4ec82cfe1220df21be28a38bb5a2ad2c894e277d27"
	)]
	#[test_case(name = "nullkey",
		"",
		"Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
		"4a9e58b6eb70cb3c3ecf5e28fcfd80d9864995250fe7efd08933c3ea29bf4ca3"
	)]
	#[test_case(name = "symbols",
		"abc123/?",
		"12345!@#%^&*()",
		"a925cd90b96eed56b49bd4f359fb30007a5d332ed75f9cf4194c1fa005230e59"
	)]
	#[test_case(name = "very_long_input",
		"this_key_is_too_long_paaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaading",
		"Supercalifragilisticexpialidocious, even though the sound of it is something quite atrocious, if you say it loud enough, you'll always sound precocious.",
		"97c79f894d8684a648a1f30e0a49074111f4c83313f0203b11053beef145239c"
	)]
	fn hmac_sha256(key: &str, input: &str, expected: &str) {
		let result = super::hmac(sha256::hash, key.as_bytes(), input.as_bytes(), 64, 32);
		// assert_eq!(format!("{}", result), expected)
	}
}
