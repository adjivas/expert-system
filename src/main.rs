// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

fn main () {
    let mut axioms = expert_sys::Set::default();
    axioms.set_imply('b', 'a');
    axioms.set_imply('c', 'b');

    let mut solver = expert_sys::Solver::new(&axioms);
    //solver.set_branch_imply('e', axioms.get_exp('c').unwrap());
    println!("{}", solver);

    //let mut rules = Vec::<std::rc::Rc<expert_sys::Exp>>::new();
}
