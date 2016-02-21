use ops::{Set, ImplyPtr};

pub struct ParseResult {
	/// List of initial_facts initialize to true.
    pub initial_facts: Vec<Set>,

    /// Instruction trees.
    pub instrs: Vec<ImplyPtr>,

    /// The axiom for which to query value at the end of the computation.
    pub query: Vec<char>
}

impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
        	initial_facts: Vec::new(),
        	instrs: Vec::new(),
        	query: Vec::new()
        }
    }

    pub fn add_set(&mut self, new_set: Set) {
        self.initial_facts.push(new_set);
    }

    pub fn add_instrs(&mut self, new_instrs: ImplyPtr) {
        self.instrs.push(new_instrs);
    }

    pub fn add_request(&mut self, new_req: &str) {
    	let req = new_req.chars().next().unwrap();
        for c in &self.query {
            if *c == req {
                return ;
            }
        }
        self.query.push(req);
    }
}
