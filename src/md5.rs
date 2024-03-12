use std::mem::size_of;

const BLOCK_SIZE: usize = 64;	// 512 bits
const DIGEST_SIZE: usize = 16;	// 128 bits

const S: [u32; 64] = [
	7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
	5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
	4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
	6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
];

const K: [u32; 64] = [
	0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
	0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
	0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
	0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
	0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
	0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
	0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
	0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
	0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
	0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
	0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
	0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
	0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
	0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
	0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391	
];

pub fn hash(message: &[u8]) -> Vec<u8> {
	// MD5 works with blocks of 512 bits
	// from the original message we will append a single 1
	// then a sequence of 0 for padding
	// and finally the message length as a 64bits integer
	let bitlen = message.len() * 8;
	let padded = crate::align!(bitlen + 1 + 64, 512);
	let bytes = padded / 8;

	let mut input = Vec::with_capacity(bytes);
	input.extend_from_slice(message);							// original message
	input.push(1 << 7);											// 1 bit
	input.resize(bytes - std::mem::size_of::<u64>(), 0);		// padding with 0
	input.extend_from_slice(&(bitlen as u64).to_le_bytes());	// message length in bits (LSB)

	let mut digest: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
	for i in (0..bytes).step_by(BLOCK_SIZE) {
		let mut a = digest[0];
		let mut b = digest[1];
		let mut c = digest[2];
		let mut d = digest[3];

		// Reinterpret the current block as a buffer of u32
		// it is safe because 512 is a multiple of both 8 and 32
		let words = unsafe {
			let block = &input[i..][..BLOCK_SIZE];
			std::slice::from_raw_parts(block.as_ptr() as *const u32, BLOCK_SIZE)
		};

		for j in 0..BLOCK_SIZE {
			let (mut f, g) = match j / 16 {
				0 => ((b & c) | (!b & d), j),
				1 => ((d & b) | (!d & c), (5 * j + 1) % 16),
				2 => (b ^ c ^ d,  (3 * j + 5) % 16),
				_ => (c ^ (b | !d), (7 * j) % 16)
			};

			f = f.wrapping_add(a).wrapping_add(K[j]).wrapping_add(words[g]);
			a = d;
			d = c;
			c = b;
			b = b.wrapping_add(f.rotate_left(S[j]));		
		}

		digest[0] = digest[0].wrapping_add(a);
		digest[1] = digest[1].wrapping_add(b);
		digest[2] = digest[2].wrapping_add(c);
		digest[3] = digest[3].wrapping_add(d);
	}

	// Clone the digest to a Vec<u8>
	let mut output = Vec::with_capacity(DIGEST_SIZE);
	for uint32 in digest {
		output.extend_from_slice(&uint32.to_le_bytes());
	}

	output
}

#[cfg(test)]
mod tests {
	use crate::hex::FromHexString;
	use ntest::test_case;

	#[test_case("d41d8cd98f00b204e9800998ecf8427e", "")]
	#[test_case("35f1d6de0302e2086a4e472266efb3a9", "42 is nice\n")]
	#[test_case("e20c3b973f63482a778f3fd1869b7f25", "Pity the living.\n")]
	#[test_case("53d53ea94217b259c11a5a2d104ec58a", "And above all,\n")]
	#[test_case("3553dc7dc5963b583c056d1b9fa3349c", "be sure to handle edge cases carefully\n")]
	#[test_case("dcdd84e0f635694d2a943fa8d3905281", "but eventually you will understand\n")]
	fn md5(expected: &str, input: &str) {
		assert_eq!(super::hash(input.as_bytes()), expected.decode_hex().unwrap());
	}
}
