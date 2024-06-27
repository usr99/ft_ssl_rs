use super::Hash;
use std::num::Wrapping;

const K: [u32; 64] = [
	0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
	0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
	0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
	0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
	0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
	0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
	0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
	0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

pub struct SHA256;
impl Hash for SHA256 {
	// 512 bits
	const BLOCK_SIZE: usize = 64;
	// 256 bits
	const DIGEST_SIZE: usize = 32;

	type Digest = [u8; Self::DIGEST_SIZE];

	fn hash(message: &[u8]) -> Self::Digest {
		// SHA256 works with blocks of 512 bits
		// from the original message we will append a single 1
		// then a sequence of 0 for padding
		// and finally the message length as a 64bits integer
		let bitlen = message.len() * 8;
		let padded = crate::align!(bitlen + 1 + 64, 512);
		let bytes = padded / 8;

		let mut input = Vec::with_capacity(bytes);
		input.extend_from_slice(message); // original message
		input.push(1 << 7); // 1 bit
		input.resize(bytes - std::mem::size_of::<u64>(), 0); // padding with 0
		input.extend_from_slice(&(bitlen as u64).to_be_bytes()); // message length in bits (MSB)

		let mut digest: [u32; 8] = [
			0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
			0x5be0cd19
		];
		for i in (0..bytes).step_by(Self::BLOCK_SIZE) {
			// Reinterpret the current block as a buffer of u32
			// it is safe because 512 is a multiple of both 8 and 32
			let words = unsafe {
				let block = &input[i..][..Self::BLOCK_SIZE];
				std::slice::from_raw_parts(block.as_ptr() as *const u32, Self::BLOCK_SIZE)
			};

			#[allow(non_snake_case)]
			let mut W = [0u32; 64];

			for j in 0..16 {
				W[j] = words[j].to_be();
			}

			for j in 16..Self::BLOCK_SIZE {
				let s0 = W[j - 15].rotate_right(7) ^ W[j - 15].rotate_right(18) ^ (W[j - 15] >> 3);
				let s1 = W[j - 2].rotate_right(17) ^ W[j - 2].rotate_right(19) ^ (W[j - 2] >> 10);
				W[j] = W[j - 16]
					.wrapping_add(s0)
					.wrapping_add(W[j - 7])
					.wrapping_add(s1);
			}

			let mut a = digest[0];
			let mut b = digest[1];
			let mut c = digest[2];
			let mut d = digest[3];
			let mut e = digest[4];
			let mut f = digest[5];
			let mut g = digest[6];
			let mut h = digest[7];

			for j in 0..Self::BLOCK_SIZE {
				let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
				let ch = (e & f) ^ (!e & g);
				let tmp1 = h
					.wrapping_add(s1)
					.wrapping_add(ch)
					.wrapping_add(K[j])
					.wrapping_add(W[j]);
				let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
				let maj = (a & b) ^ (a & c) ^ (b & c);
				let tmp2 = s0.wrapping_add(maj);

				h = g;
				g = f;
				f = e;
				e = d.wrapping_add(tmp1);
				d = c;
				c = b;
				b = a;
				a = tmp1.wrapping_add(tmp2);
			}

			digest[0] = digest[0].wrapping_add(a);
			digest[1] = digest[1].wrapping_add(b);
			digest[2] = digest[2].wrapping_add(c);
			digest[3] = digest[3].wrapping_add(d);
			digest[4] = digest[4].wrapping_add(e);
			digest[5] = digest[5].wrapping_add(f);
			digest[6] = digest[6].wrapping_add(g);
			digest[7] = digest[7].wrapping_add(h);
		}

		// Enforce big endian byte order
		for uint32 in digest.iter_mut() {
			*uint32 = uint32.to_be();
		}

		// Convert [u32; 8] into [u8; 32]
		unsafe { std::mem::transmute(digest) }
	}
}

#[cfg(test)]
mod tests {
	use crate::hex::FromHexString;
	use crate::Hash;
	use ntest::test_case;

	#[test_case("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", "")]
	#[test_case(
		"1ceb55d2845d9dd98557b50488db12bbf51aaca5aa9c1199eb795607a2457daf",
		"https://www.42.fr/\n"
	)]
	#[test_case(
		"a5482539287a4069ebd3eb45a13a47b1968316c442a7e69bc6b9c100b101d65d",
		"42 is nice\n"
	)]
	#[test_case(
		"eadea59b233a79582242c1e4bb78db4faedec330fddef4828f9c8018b2fab666",
		"sha256 is my favorite algorithm."
	)]
	#[test_case(
		"145df9000eb3bb2ffc46c68e1d9795129d62af0e9a09b55dc5f3293b03e83b3f",
		"i hope these tests will be useful"
	)]
	#[test_case(
		"0c9478ffc28e628885a8db85777a39598086ba333190b3a8031adbd9de730af7",
		"i hope these tests will ae useful"
	)]
	fn sha256(expected: &str, input: &str) {
		let result = super::SHA256::hash(input.as_bytes());
		let expected = expected.decode_hex().unwrap();

		assert_eq!(result.as_ref(), &expected);
	}
}
