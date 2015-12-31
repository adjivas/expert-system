// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

/// The `main` start function examples how to
/// use Solver with dependency.

fn main () {
    let mut axioms = expert_sys::Set::default();
    let mut solver = expert_sys::Solver::new(&mut axioms);

    solver.set_branch_imply ('a',
        axioms.get_exp('b').unwrap(),
    );
    println!("{}", solver);
}
