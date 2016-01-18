// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

/// The `main` start function examples how to use Axiom.

fn main () {
    let mut axioms = expert_sys::Set::default();

    axioms.set_value('a', true);
    println!("{}", axioms);
    println!("{}", axioms['a']);
}
