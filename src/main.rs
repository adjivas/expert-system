// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;
#[macro_use]
extern crate expert_sys;

mod parse;
mod tokenizer;
mod exp;
mod fc_string;
mod axiom;
mod rules;
mod set;
mod macros;

#[cfg(test)]
mod test_parse;
mod ops;

use std::fs::File;
use std::env;
use std::io::prelude::*;
use parse::{Parser};
pub use exp::{Exp};

fn file_as_string(filename: &String) -> String {
    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    s
}

/// Return the file name to parse in this execution.
fn args_parse() -> String {
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("usage: {} file_name", args[0]);
		std::process::exit(1)
	}
	args[1].clone()
}

fn main () {
	let filename = args_parse();
	let instructions_str = file_as_string(&filename);
	let instructions = Parser::parse(&instructions_str);
}
