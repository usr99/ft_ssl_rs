use crate::Hash;

fn hash(input: &[u8]) -> Hash {
	Hash(Vec::new())
}

#[cfg(test)]
mod tests {
	use ntest::test_case;

	#[test_case("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", "")]
	#[test_case("1ceb55d2845d9dd98557b50488db12bbf51aaca5aa9c1199eb795607a2457daf", "https://www.42.fr/")]
	#[test_case("b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f", "42 is nice")]
	#[test_case("eadea59b233a79582242c1e4bb78db4faedec330fddef4828f9c8018b2fab666", "sha256 is my favorite algorithm.")]
	#[test_case("145df9000eb3bb2ffc46c68e1d9795129d62af0e9a09b55dc5f3293b03e83b3f", "i hope these tests will be useful")]
	#[test_case("0c9478ffc28e628885a8db85777a39598086ba333190b3a8031adbd9de730af7", "i hope these tests will ae useful")]
	fn sha256(expected: &str, input: &str) {
		assert_eq!(crate::hash!(input), expected);
	}
}