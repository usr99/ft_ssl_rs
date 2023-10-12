use std::collections::HashMap;

const ALPHABET: [char; 64] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];

pub fn encode(buf: &[u8]) -> Vec<u8> {
	const MASK: u8 = 0b11111100;
	
	let mut encoded = Vec::new();
	let mut equal_signs = 0;
	let mut byte = 0;
	let mut bit = 0;

	while byte < buf.len() {
		let a = buf[byte] & (MASK >> bit);
		let mut character = a >> std::cmp::max(0, 2 - bit);

		bit += 6;
		if bit >= 8 {
			bit %= 8;
			byte += 1;

			if bit != 0 {
				let mut b = 0;
				if byte < buf.len() {
					b = buf[byte] & (MASK << (6 - bit));
				} else {
					equal_signs = bit / 2;
				}
				character = (a << bit) | (b >> (8 - bit));
			}
		}

		encoded.push(ALPHABET[character as usize] as u8);
	}

	for _ in 0..equal_signs {
		encoded.push('=' as u8);
	}

	encoded
}

pub fn decode(buf: &[u8]) -> Vec<u8> {
	const MASK: u8 = 0b00111111;
	let alphabet = ALPHABET.iter().enumerate().map(|(id, letter)| (*letter as u8, id as u8)).collect::<HashMap<_, _>>();
	
	let mut decoded = Vec::new();
	let mut tmp = 0;
	let mut byte = 0;
	let mut bit = 0;

	for byte in buf.iter() {
		if *byte == '=' as u8 {
			if bit < 2 || tmp == 0 {
				break ;
			}

			tmp <<= (8 - bit);
			decoded.push(tmp);
			break ;
		}
		
		let plain = match alphabet.get(&byte) {
			Some(value) => *value & &MASK,
			None => break
		};
		
		if bit == 0 {
			tmp = plain;
		} else {
			let left = 8 - bit;
			tmp = (tmp << left) | (plain >> 6 - left);
		
			if left <= 6 {
				decoded.push(tmp);
				if left < 6 {
					let shift = (8 - (6 - left));
					tmp = plain << shift >> shift; 
				}
			}
		}

		bit += 6;
		if bit >= 8 {
			bit %= 8;
		}
	}

	decoded
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
		"TXkgc29uLiBUaGUgZGF5IHlvdSB3ZXJlIGJvcm4sIHRoZSB2ZXJ5IGZvcmVzdHMgb2YgTG9yZGFlcm9uIHdoaXNwZXJlZCB0aGUgbmFtZSwgQXJ0aGFz",
		"My son. The day you were born, the very forests of Lordaeron whispered the name, Arthas")]
	#[test_case(
		"YWI/Y2Q+Cg==",
		"ab?cd>\n")]
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
		"TXkgc29uLiBUaGUgZGF5IHlvdSB3ZXJlIGJvcm4sIHRoZSB2ZXJ5IGZvcmVzdHMgb2YgTG9yZGFlcm9uIHdoaXNwZXJlZCB0aGUgbmFtZSwgQXJ0aGFz")]
	#[test_case(
		"ab?cd>\n",
		"YWI/Y2Q+Cg==")]
	fn base64_decode(expected: &str, input: &str) {
		let decoded = super::decode(input.as_bytes());
		assert_eq!(expected, std::str::from_utf8(&decoded).unwrap());
	}
}