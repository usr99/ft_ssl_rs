use std::io::Write;
use ntest::test_case;
use assert_cmd::Command;

#[test_case(
	"(stdin)= 35f1d6de0302e2086a4e472266efb3a9\n",
	"42 is nice\n",
	""
)]
#[test_case(
	"(\"42 is nice\")= 35f1d6de0302e2086a4e472266efb3a9\n",
	"42 is nice\n",
	"-p"
)]
#[test_case(
	"e20c3b973f63482a778f3fd1869b7f25\n",
	"Pity the living.\n",
	"-qr"
)]
fn stdin(expected: &str, input: &str, args: &str) {
	let mut cmd = Command::cargo_bin("ft_ssl").unwrap();
	
	let mut cmd = cmd.arg("md5").write_stdin(input);
	if args.len() != 0 {
		cmd = cmd.arg(args);
	}
	
	cmd.assert().success().stdout(expected);
}

#[test_case(
	"MD5 (/tmp/file) = 53d53ea94217b259c11a5a2d104ec58a\n",
	"And above all,\n",
	""
)]
#[test_case(
	"53d53ea94217b259c11a5a2d104ec58a /tmp/file\n",
	"And above all,\n",
	"-r"
)]
#[test_case(
	"53d53ea94217b259c11a5a2d104ec58a\n",
	"And above all,\n",
	"-qr"
)]
fn file(expected: &str, file_content: &str, args: &str) {
	let mut cmd = Command::cargo_bin("ft_ssl").unwrap();

	const FILENAME: &str = "file";
	let mut dir = std::env::temp_dir();

	dir.push(FILENAME);
	let mut file = std::fs::File::create(&dir).unwrap();
	file.write_all(file_content.as_bytes()).unwrap();

	let mut cmd = cmd.arg("md5");
	if args.len() != 0 {
		cmd = cmd.arg(args);
	}
	cmd.arg(dir.to_str().unwrap())
		// test that stdin is ignored if -p is not specified
		.write_stdin("some of this will not make sense at first\n")
		.assert().success().stdout(expected);
}

#[test_case(
	"MD5 (\"pity those that aren't following baerista on spotify.\") = a3c990a1964705d9bf0e602f44572f5f\n",
	"pity those that aren't following baerista on spotify.",
	"-s"
)]

#[test_case(
	"acbd18db4cc2f85cedef654fccc4a4d8 \"foo\"\n",
	"foo",
	"-rs"
)]
#[test_case(
	"acbd18db4cc2f85cedef654fccc4a4d8\n",
	"foo",
	"-qrs"
)]
fn strings(expected: &str, input: &str, args: &str) {
	let mut cmd = Command::cargo_bin("ft_ssl").unwrap();
	
	cmd.arg("md5").arg(args).arg(input)
		.assert().success().stdout(expected);
}

#[test_case(
	"(\"be sure to handle edge cases carefully\")= 3553dc7dc5963b583c056d1b9fa3349c\n\
		MD5 (/tmp/file) = 53d53ea94217b259c11a5a2d104ec58a\n",
	"be sure to handle edge cases carefully\n",
	"-p",
	""
)]
#[test_case(
	"(\"but eventually you will understand\")= dcdd84e0f635694d2a943fa8d3905281\n\
		53d53ea94217b259c11a5a2d104ec58a /tmp/file\n",
	"but eventually you will understand\n",
	"-pr",
	""
)]
#[test_case(
	"(\"GL HF let's go\")= d1e3cc342b6da09480b27ec57ff243e2\n\
		acbd18db4cc2f85cedef654fccc4a4d8 \"foo\"\n\
		53d53ea94217b259c11a5a2d104ec58a /tmp/file\n",
	"GL HF let's go\n",
	"-prs",
	"foo"
)]
#[test_case(
	"just to be extra clear\n\
		3ba35f1ea0d170cb3b9a752e3360286c\n\
		acbd18db4cc2f85cedef654fccc4a4d8\n\
		53d53ea94217b259c11a5a2d104ec58a\n",
	"just to be extra clear\n",
	"-pqrs",
	"foo"
)]
fn complex(expected: &str, stdin: &str, args: &str, string: &str) {
	let mut cmd = Command::cargo_bin("ft_ssl").unwrap();

	const FILENAME: &str = "file";
	let mut dir = std::env::temp_dir();

	dir.push(FILENAME);
	let mut file = std::fs::File::create(&dir).unwrap();
	file.write_all("And above all,\n".as_bytes()).unwrap();

	let mut cmd = cmd.arg("md5").arg(args);
	if string.len() != 0 {
		cmd = cmd.arg(string);
	}
	cmd.arg(dir.to_str().unwrap())
		.write_stdin(stdin)
		.assert().success().stdout(expected);
}
