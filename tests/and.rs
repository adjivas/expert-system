// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

/// The `test_example` checks the demonstration.

#[test]
fn test_example () {
    let mut axioms: expert_sys::Set = expert_sys::Set::default();

    axioms.set_value('a', true);
    axioms.set_value('b', true);
    let a_and_b = expert_sys::ops::And::new (
        axioms.get_exprs('a').unwrap(),
        axioms.get_exprs('b').unwrap()
    );
    let b_and_c = expert_sys::ops::And::new (
        axioms.get_exprs('b').unwrap(),
        axioms.get_exprs('c').unwrap()
    );
}
