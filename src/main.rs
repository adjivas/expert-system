// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

fn file_as_string(filename: &String) -> String {
    use std::io::prelude::*;
    let mut f = std::fs::File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    s
}

/// Return the file name to parse in this execution.
fn args_parse() -> String {
	let args: Vec<_> = std::env::args().collect();
	if args.len() < 2 {
		println!("usage: {} file_name", args[0]);
		std::process::exit(1)
	}
	args[1].clone()
}

fn main () {
	let filename = args_parse();
	let instructions_str = file_as_string(&filename);
    let rules_parse: Option<expert_sys::Rules> = expert_sys::Parser::parse(&instructions_str);

    if let Some(rules) = rules_parse {
        rules.get_instrs('a');
    }
}
