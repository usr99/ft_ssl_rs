use super::{PRF, utils::xor};

fn pbkdf2<F: PRF>(password: &[u8], salt: &[u8], iterations: u32, key_length: u32) -> Vec<u8> {
	let block_length = F::OUTPUT_SIZE;
	let block_count = (key_length as usize + block_length - 1) / block_length; 

	let mut key = vec![0u8; block_count * block_length];
	let blocks = key.chunks_mut(block_length);

	for (idx, blk) in blocks.enumerate() {
		let initial_salt = [salt, (idx as u32 + 1).to_be_bytes().as_ref()].concat();

		let mut init = F::randgen(password, &initial_salt);
		blk.copy_from_slice(init.as_ref());

		let mut last = init;
		for _ in 1..iterations {
			last = F::randgen(password, last.as_ref());
			blk.copy_from_slice(xor(blk, last.as_ref()).as_slice());
		}
	}

	key.resize(key_length as usize, 0);
	return key;
}

#[cfg(test)]
mod test {
	use crate::{hex::ToHexString, HMAC, SHA256};
	use ntest::test_case;

	#[test_case(
		name = "iter_1",
		"password",
		"salt",
		1,
		32,
		"120fb6cffcf8b32c43e7225256c4f837a86548c92ccc35480805987cb70be17b"
	)]
	#[test_case(
		name = "iter_4096",
		"password",
		"salt",
		4096,
		32,
		"c5e478d59288c841aa530db6845c4c8d962893a001ce4e11a4963873aa98134a"
	)]
	#[test_case(
		name = "shorter_key",
		"password",
		"salt",
		256,
		8,
		"951ad61af6eb7d81"
	)]
	#[test_case(
		name = "truncate_key",
		"password",
		"salt",
		256,
		29,
		"951ad61af6eb7d8126db061d25488e844625313aee9ec511e87c9750bd"
	)]
	fn pbkdf2_hmac_sha256(passwd: &str, salt: &str, iter: u32, keylen: u32, expected: &str) {
		let result =
			super::pbkdf2::<HMAC<SHA256>>(passwd.as_bytes(), salt.as_bytes(), iter, keylen);

		assert_eq!(result.encode_hex(), expected)
	}
}
