#![allow(unused)]
use ft_ssl::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct CLI {
	/// encryption mode (md5, sha256)
	command: String,

	/// echo STDIN to STDOUT
	#[arg(short = 'p')]
	echo_stdin: bool,
	
	/// quiet mode
	#[arg(short)]
	quiet: bool,
	
	/// reverse the format of the output
	#[arg(short)]
	reverse: bool,
	
	/// additionnal string input
	#[arg(short)]
	strings: Vec<String>,

	/// file input
	file: Option<String>,
}

fn main() {
	let cli = CLI::parse();

    dbg!(&cli);
}
