// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

fn main () {
    let mut axioms = expert_sys::Set::default();
    println!("{}", axioms);
    axioms.set_imply('b', 'a');
    axioms.set_imply('c', 'b');
    println!("{}", axioms);

    let mut solver = expert_sys::Solver::new(&axioms);
    solver.set_branch_imply('e', axioms.get_exp('c').unwrap());
    println!("{}", solver);
}

/*use expert_sys::Set;
use expert_sys::ops::Unary;
use expert_sys::ops::Binary;

fn main () {
    let mut axioms = Set::default();
    let a_and_b = expert_sys::ops::And::new (
        &mut axioms['a'],
        &mut axioms['b'],
    );

    expert_sys::command::escutcheon();
    *axioms['a'] = true;
    *axioms['b'] = true;
    println!("{}", a_and_b);
}*/
