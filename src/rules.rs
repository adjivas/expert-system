use Set;
use solver::exp::Exp;
use std::rc::Rc;
use ops::Imply;

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

    fn get_exprs_imply(&self, query: Rc<Exp>) -> Option<Rc<Exp>> {
        if let (Some(res), _) = self.instrs.iter().fold((
            None,
            query.get_ident()
        ), |(res, acc), ins|
            if res.is_none() {
                if let Some(instruction) = Rc::downgrade(ins).upgrade() {
                    if instruction.get_ident_right() == acc {
                        (Some(instruction.get_exprs_left()), acc)
                    }
                    else { (res, acc) }
                }
                else { (res, acc) }
            }
            else { (res, acc) }
        ) {
            res
        }
        else {
            None
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

    pub fn get_instrs(&self) -> &Vec<Rc<Exp>> {
        &self.instrs
    }
}
