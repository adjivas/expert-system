use ops::{Set, Exp, ExpPtr};
use std::rc::Rc;

pub struct ParseResult {
	/// List of initial_facts initialize to true.
    initial_facts: Vec<Set>,

    /// Instruction trees.
    instrs: Vec<ExpPtr>,

    /// The axiom for which to request value at the end of the computation.
    request: Vec<char>
}

impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
        	initial_facts: Vec::new(),
        	instrs: Vec::new(),
        	request: Vec::new()
        }
    }

    pub fn add_set(&mut self, new_set: Set) {
        self.initial_facts.push(new_set);
    }

    pub fn add_instrs(&mut self, new_instrs: ExpPtr) {
        self.instrs.push(new_instrs);
    }

    pub fn add_request(&mut self, new_req: &str) {
    	let req = new_req.chars().next().unwrap();
        for c in &self.request {
            if *c == req {
                return ;
            }
        }
        self.request.push(req);
    }

    pub fn get_instrs(&self) -> &Vec<ExpPtr> {
        &self.instrs
    }

    pub fn get_initial_value(&self) -> &Vec<Set> {
        &self.initial_facts
    }
}
