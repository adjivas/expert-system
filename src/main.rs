// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

<<<<<<< HEAD
extern crate expert_sys;
=======
extern crate regex;

mod parser;
mod parse_result;
mod solver;
mod ops;

use std::fs::File;
use std::env;
use std::io::prelude::*;
use parser::{Parser};
use ops::{Exp, Set, ImplyPtr};
use std::collections::HashMap;
>>>>>>> origin/guillaume

fn file_as_string(filename: &String) -> String {
    use std::io::prelude::*;
    let mut f = std::fs::File::open(filename).unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
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

<<<<<<< HEAD
fn main () {
	let filename = args_parse();
	let instructions_str = file_as_string(&filename);
    let rules_parse: Option<expert_sys::Rules> = expert_sys::Parser::parse(&instructions_str);

    if let Some(rules) = rules_parse {
        rules.resolve();
=======
fn resolve_and_print(
	deps: &HashMap<char, ImplyPtr>,
	initial_facts: &Set
) {
	let initial_facts_str = initial_facts.true_fact_str();
	println!("\nWith true facts : {}", initial_facts_str);
    for (key, instr) in deps {
    	let mut final_facts = Set::new();
    	instr.borrow().solve(initial_facts, &mut final_facts);
    	let value = final_facts.get_value(*key);
    	println!("For {} value is {}", key, value);
    }
}

fn main () {
	let filename = args_parse();
	let instructions_str = file_as_string(&filename);
	let parsed = Parser::parse(&instructions_str);
	if parsed.is_none() {
		println!("Parse error");
		return ;
	}
	let parsed = parsed.unwrap();
    let deps = solver::solve(&parsed);
    println!("Query dependences:");
    for (key, value) in &deps {
    	println!("For {} dependence tree is: {}",
    			key, value.borrow().get_ident().unwrap());
    }
    println!("\nSolution according to those dependences:");
    for initial_facts in &parsed.initial_facts {
        resolve_and_print(&deps, initial_facts);
>>>>>>> origin/guillaume
    }
}
