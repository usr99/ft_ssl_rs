use std::cmp::Ordering;

const IPAD: u8 = 0x36;
const OPAD: u8 = 0x5c;

// See https://en.wikipedia.org/wiki/HMAC
pub fn hmac(hash: fn(&[u8]) -> Vec<u8>, key: &[u8], message: &[u8], block_size: usize) -> Vec<u8> {
	let key = prepare_key(hash, key, block_size);

	let mut inner = xor(&vec![IPAD; block_size], &key);
	inner.extend_from_slice(message);

	let mut outer = xor(&vec![OPAD; block_size], &key);
	outer.extend_from_slice(hash(&inner).as_ref());

	hash(&outer)
}

// Apply XOR on 2 byte sequences
// the two sequences must be of equal length
fn xor(left: &[u8], right: &[u8]) -> Vec<u8> {
	assert!(left.len() == right.len());
	left.iter().zip(right.iter()).map(|(a, b)| a ^ b).collect()
}

fn prepare_key(hash: fn(&[u8]) -> Vec<u8>, key: &[u8], block_size: usize) -> Vec<u8> {
	// Hash the key if it's too long
	let mut key = match key.len().cmp(&block_size) {
		Ordering::Greater => hash(&key),
		_ => key.to_vec(),
	};

	// The key must be of length block_size
	// append 0s if necessary
	key.resize(block_size, 0);

	key
}

#[cfg(test)]
mod test {
	use crate::{hex::ToHexString, sha256};
	use ntest::test_case;

	#[test_case(
		name = "sha256_basic",
		"password",
		"The quick brown fox jumps over the lazy dog.",
		"6d0ed1a0c1a5e0f9fa971c4ec82cfe1220df21be28a38bb5a2ad2c894e277d27"
	)]
	#[test_case(
		name = "nullkey",
		"",
		"Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
		"4a9e58b6eb70cb3c3ecf5e28fcfd80d9864995250fe7efd08933c3ea29bf4ca3"
	)]
	#[test_case(
		name = "symbols",
		"abc123/?",
		"12345!@#%^&*()",
		"a925cd90b96eed56b49bd4f359fb30007a5d332ed75f9cf4194c1fa005230e59"
	)]
	#[test_case(
		name = "very_long_input",
		"this_key_is_too_long_paaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaading",
		"Supercalifragilisticexpialidocious, even though the sound of it is something quite atrocious, if you say it loud enough, you'll always sound precocious.",
		"ee4fa7f42be5379063a8c8aeea04ae6b29021acf01fba578e55529ce12f4f272"
	)]
	#[test_case(
		name = "key_256_bit",
		"GuVGvcBnbjQNxUfAusQDuUcWSSSbIXqhFweoBuSToKCUwPqPCuTVhHViEAtZdwzI",
		"this text need to be encoded with a key as long as a sha256 digest",
		"3249254036318733e326f225ebcd99e58b55cd31b43d9808220f9d7d2e6aa871"
	)]
	fn hmac_sha256(key: &str, input: &str, expected: &str) {
		let result = super::hmac(sha256::hash, key.as_bytes(), input.as_bytes(), 64);
		assert_eq!(result.encode_hex(), expected)
	}

	#[test]
	fn hmac_collision() {
		let input = "both passwords should produce the same output because of the way hmac prepares keys".as_bytes();
		let password = "plnlrtfpijpuhqylxbgqiiyipieyxvfsavzgxbbcfusqkozwpngsyejqlmjsytrmd".as_bytes();
		let digest = sha256::hash(password);

		assert_ne!(password, digest);

		let from_password = super::hmac(sha256::hash, password, input, 64);
		let from_digest = super::hmac(sha256::hash, &digest, input, 64);

		assert_eq!(from_password, from_digest)
	}
}
