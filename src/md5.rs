use crate::Hash;

fn hash(input: &[u8]) -> Hash {
	Hash(Vec::new())
}

#[cfg(test)]
mod tests {
	use ntest::test_case;

	#[test_case("d41d8cd98f00b204e9800998ecf8427e", "")]
	#[test_case("35f1d6de0302e2086a4e472266efb3a9", "42 is nice")]
	#[test_case("e20c3b973f63482a778f3fd1869b7f25", "Pity the living.")]
	#[test_case("53d53ea94217b259c11a5a2d104ec58a", "And above all,")]
	#[test_case("3553dc7dc5963b583c056d1b9fa3349c", "be sure to handle edge cases carefully")]
	#[test_case("dcdd84e0f635694d2a943fa8d3905281", "but eventually you will understand")]
	fn md5(expected: &str, input: &str) {
		assert_eq!(crate::hash!(input), expected);
	}
}
