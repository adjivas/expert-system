// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Parser;
use expert_sys::Tokenizer;
use expert_sys::Rules;

#[cfg(test)]
use expert_sys::ops;

use std::fs::File;
use std::env;
use std::io::prelude::*;

use std::rc::Rc;
use expert_sys::Exp;
use expert_sys::ops::Imply;

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

/*pub fn get_from (
    rules: &Vec<Rc<Imply>>,
    ident: &String,
) -> Option<String> {
    for rule in rules {
        match (*rule.get_ident_from(), *rule.get_ident_to()) {
            (Some(ref from), Some(ref to)) if to == ident => return get_from(rules, from),
            _ => continue ,
        }
    }
    Some(ident.clone())
}
*/
fn main () {
	let filename = args_parse();
	let instructions_str = file_as_string(&filename);

    let rules: Option<Rules> = Parser::parse(&instructions_str);
	if let Some(rules) = rules {
//        rules.resolve('a');

//            println!("{} {}", rule, get_from(&rules, &rule.get_ident_to().unwrap()).unwrap());

    }
}
