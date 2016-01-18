// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Exp;

#[test]
fn test_example () {
    let mut axioms: expert_sys::Set = expert_sys::Set::default();

    let not_a: std::rc::Rc<expert_sys::ops::Not> = expert_sys::ops::Not::new (
        axioms.get_exp('a').unwrap(),
    );
    let not_not_a: std::rc::Rc<expert_sys::ops::Not> = expert_sys::ops::Not::new (
        not_a
    );
    let not_b: std::rc::Rc<expert_sys::ops::Not> = expert_sys::ops::Not::new (
        axioms.get_exp('b').unwrap(),
    );
    let not_c: std::rc::Rc<expert_sys::ops::Not> = expert_sys::ops::Not::new (
        axioms.get_exp('c').unwrap(),
    );
    assert_eq!(not_not_a.get_value(), Some(false));
}
