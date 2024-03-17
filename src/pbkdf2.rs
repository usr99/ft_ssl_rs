use super::PRF;

fn pbkdf2<F: PRF>(passwd: &[u8], salt: &[u8], iter: usize, keylen: usize) -> Vec<u8> { vec![] }

#[cfg(test)]
mod test {
	use crate::{hex::ToHexString, HMAC, SHA256};
	use ntest::test_case;

	#[test_case(
		name = "iter_1",
		"password",
		"salt",
		1,
		256,
		"120fb6cffcf8b32c43e7225256c4f837a86548c92ccc35480805987cb70be17"
	)]
	#[test_case(
		name = "iter_4096",
		"password",
		"salt",
		4096,
		256,
		"c5e478d59288c841aa530db6845c4c8d962893a001ce4e11a4963873aa98134a"
	)]
	#[test_case(
		name = "keylen_64",
		"password",
		"salt",
		1000000,
		64,
		"a4c826c9511b92f4"
	)]
	fn pbkdf2_hmac_sha256(passwd: &str, salt: &str, iter: usize, keylen: usize, expected: &str) {
		let result =
			super::pbkdf2::<HMAC<SHA256>>(passwd.as_bytes(), salt.as_bytes(), iter, keylen);

		assert_eq!(result.encode_hex(), expected)
	}
}
