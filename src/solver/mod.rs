// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(test)]
mod test_solve;

use parse_result::{ParseResult};
use ops::{ImplyPtr, Or, Imply, Axiom};
use std::collections::HashMap;

fn merge_instrs(dependences: Vec<ImplyPtr>, query: char) -> ImplyPtr {
    let mut from = dependences[0].borrow().from.clone();
    for i in 1..dependences.len() {
        let next = dependences[i].borrow().from.clone();
        from = Or::new(from, next);
    }
    Imply::new_ptr(from, Axiom::new_ptr(query))
}

/// Generate one rules for the axiom `query` which is the concatenation of all
/// the rules which change query.
fn generate_query_tree(query: char, instrs: &Vec<ImplyPtr>) -> ImplyPtr {
	// list all instrctions which change the value of `query`
	let mut dependences = Vec::new();
	for instr in instrs {
		if instr.borrow().imply_axiom(query) {
			dependences.push(instr.clone());
		}
	}

	// generate the result according to le list of dependences
	match dependences.len() {
	    0 => Imply::new_ptr(Axiom::new_ptr(query), Axiom::new_ptr(query)),
	    _ => merge_instrs(dependences, query)
	}
}

pub fn solve(instrs: ParseResult) -> HashMap<char, ImplyPtr> {
	let mut to_return = HashMap::new();
    for query in instrs.query {
        let tree = generate_query_tree(query, &instrs.instrs);
        to_return.insert(query, tree);
    }
    to_return
}
