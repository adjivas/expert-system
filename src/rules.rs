use Set;
use solver::exp::Exp;
use std::rc::Rc;

pub struct Rules {
	/// List of initial_facts initialize to true.
    initial_facts: Set,

    /// Instruction trees.
    instrs: Vec<Rc<Exp>>,

    /// The axiom for which to request value at the end of the computation.
    request: Vec<char>
}

impl  Rules {

    pub fn new() -> Rules {
        Rules {
        	initial_facts: Set::default(),
        	instrs: Vec::new(),
        	request: Vec::new()
        }
    }

    pub fn add_set(&mut self, new_set: Set) {
        self.initial_facts = new_set;
    }

    pub fn add_instrs(&mut self, new_instrs: Rc<Exp>) {
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

    /// The `get_imply` function returns the value axiom.

    pub fn get_imply(&self, targ: &Rc<Exp>) -> Option<bool> {

        if let Some(targ_a) = Rc::downgrade(&targ).upgrade() {
            let a: Option<String> = targ_a.get_ident();
            for instr in self.instrs.iter() {
                if let Some(targ_b) = Rc::downgrade(&instr).upgrade() {
                    let b: Option<String> = targ_b.get_ident_right();
                    if a == b {
                        if let Some(expr) = targ_b.get_exprs_left() {
                            println!("debug search: {}<=>{}", targ_a, targ_b);
                            return expr.put_eval_imply(self);
                        }
                    }
                }
            }
        }

        println!("debug: {}", targ);
        targ.get_value()
    }

    pub fn get_axiom(&self, letter: char) -> Option<bool> {
        if let Some(expr) = self.initial_facts.get_exprs(letter) {
            self.get_imply(&expr)
        }
        else {
            None
        }
    }

    pub fn resolve (&self) {
        println!("debug:{}\n", self.initial_facts);
        for req in &self.request {
            match self.get_axiom(*req) {
                Some(result) => println!("{}=>{}", req, result),
                None => println!("{}=>None", req),
            }
        }
    }
}
