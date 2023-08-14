pub fn encode(buf: &[u8]) -> Vec<u8> {
	Vec::new()
}

pub fn decode(buf: &[u8]) -> Vec<u8> {
	Vec::new()
}

#[cfg(test)]
mod test {
	use ntest::test_case;

	#[test_case(
		"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gTWF1cmlzIGVnZXN0YXM=",
		"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris egestas")]
	#[test_case(
		"NDIgaXMgbmljZQ==",
		"42 is nice")]
	#[test_case(
		"TXkgc29uLiBUaGUgZGF5IHlvdSB3ZXJlIGJvcm4sIHRoZSB2ZXJ5IGZvcmVzdHMg",
		"My son. The day you were born, the very forests of Lordaeron whispered the name, Arthas")]
	fn base64_encode(expected: &str, input: &str) {
		let encoded = super::encode(input.as_bytes());
		assert_eq!(expected, std::str::from_utf8(&encoded).unwrap());
	}

	#[test_case(
		"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris egestas",
		"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gTWF1cmlzIGVnZXN0YXM=")]
	#[test_case(
		"42 is nice",
		"NDIgaXMgbmljZQ==")]
	#[test_case(
		"My son. The day you were born, the very forests of Lordaeron whispered the name, Arthas",
		"TXkgc29uLiBUaGUgZGF5IHlvdSB3ZXJlIGJvcm4sIHRoZSB2ZXJ5IGZvcmVzdHMg")]
	fn base64_decode(expected: &str, input: &str) {
		let decoded = super::encode(input.as_bytes());
		assert_eq!(expected, std::str::from_utf8(&decoded).unwrap());
	}

}