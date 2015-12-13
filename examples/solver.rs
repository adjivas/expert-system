// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Set;

fn main () {
    let mut axioms: expert_sys::Set = Set::default();
    let mut solver = expert_sys::Solver::new (&mut axioms);

    /*solver.set_imply (
        &mut axioms['a'],
        &mut axioms['b'],
    );
    solver.set_imply (
        &mut axioms['c'],
        &mut axioms['d'],
    );*/
}
