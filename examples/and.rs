// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::ops::Binary;

/// The `main` start function examples how to
/// use Axiom with door And.

fn main () {
    let mut axioms = expert_sys::Set::default();

    axioms.set_value('a', true);
    axioms.set_value('b', true);
    let a_and_b = expert_sys::ops::And::new (
        axioms.get_exp('a').unwrap(),
        axioms.get_exp('b').unwrap()
    );
    let b_and_c = expert_sys::ops::And::new (
        axioms.get_exp('b').unwrap(),
        axioms.get_exp('c').unwrap()
    );
    println!("{}, {}", a_and_b, b_and_c);
}
