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
use ops::Exp;

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
fn generate_query_tree(
	query: char,
	instrs: &Vec<ImplyPtr>
) -> Option<ImplyPtr> {
	// list all instructions which change the value of `query`
	let mut dependences = Vec::new();
	for instr in instrs {
		if instr.borrow().imply_axiom(query) {
			dependences.push(instr.clone());
		}
	}

	// merge all dependences into one
	let merged_dependences = match dependences.len() {
	    0 => return None,
	    _ => merge_instrs(dependences, query)
	};

	// resolve dependences for the `from` part of the instruction
	let mut carry_on = true;
	while carry_on {
		let from = merged_dependences.borrow().from.clone();

		// list of axioms from which the value of query depend
	    let dep_axioms = from.borrow().list_axiom();
	    carry_on = false;
	    for dep_axiom in dep_axioms {

	    	// we test if this axiom could be replace by another expression
	    	// it depends on.
	        let axiom_dependence_opt = generate_query_tree(dep_axiom, instrs);
	        match axiom_dependence_opt {
	            Some(axiom_dependence) => {
	            	carry_on = true;
	            	merged_dependences.borrow_mut().
	            		replace_axiom(dep_axiom, axiom_dependence.borrow().from.clone());
	            },
	            None => {},
	        }
	    }
	};
	Some(merged_dependences)
}

pub fn solve(instrs: &ParseResult) -> HashMap<char, ImplyPtr> {
	let mut to_return = HashMap::new();
    for query in &instrs.query {
        let tree_opt = generate_query_tree(*query, &instrs.instrs);
        let new_tree = match tree_opt {
            Some(tree) => tree,
            None => Imply::new_ptr(Axiom::new_ptr(*query), Axiom::new_ptr(*query)),
        };
        to_return.insert(*query, new_tree);
    }
    to_return
}
