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

    fn put_eval_init(&self, targ: Rc<Exp>) {
        if let Some(targ_a) = Rc::downgrade(&targ).upgrade() {
            let a: Option<String> = targ_a.get_ident();
            for instr in self.instrs.iter() {
                if let Some(targ_b) = Rc::downgrade(&instr).upgrade() {
                    let b: Option<String> = targ_b.get_ident_right();
                    if a == b {
                        if let Some(expr) = targ_b.get_exprs_left() {
                            expr.put_eval_imply(&self.instrs);
                        }
                    }
                }
            }
        }

        /*if let (Some(res), _) = self.instrs.iter().fold((
            None,
            query.get_ident()
        ), |(res, acc), ins|
            if res.is_none() {
                println!("{:?}", acc);
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
        }*/
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

    pub fn get_instrs(&self, letter: char) {
        if let Some(expr) = self.initial_facts.get_exprs(letter) {
            self.put_eval_init(expr);
        }
    }
}
