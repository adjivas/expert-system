// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

/// The `main` start function examples how to
/// use Axiom with door Xor.

fn main () {
    let mut axioms = expert_sys::Set::default();

    axioms.set_value('a', true);
    axioms.set_value('b', true);
    let a_xor_b = expert_sys::ops::Xor::new (
        axioms.get_exprs('a').unwrap(),
        axioms.get_exprs('b').unwrap()
    );
    let b_xor_c = expert_sys::ops::Xor::new (
        axioms.get_exprs('b').unwrap(),
        axioms.get_exprs('c').unwrap()
    );
    println!("{}, {}", a_xor_b, b_xor_c);
}
