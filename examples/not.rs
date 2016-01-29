// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

/// The `main` start function examples how to
/// use Axiom with door Not.

fn main () {
    let axioms = expert_sys::Set::default();
    let not_b = expert_sys::ops::Not::new (
        axioms.get_exprs('b').unwrap(),
    );
    let not_c = expert_sys::ops::Not::new (
        axioms.get_exprs('c').unwrap(),
    );
    println!("{}, {}", not_b, not_c);
}
