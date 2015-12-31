// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::ops::Unary;
use expert_sys::Exp;

#[test]
fn test_value () {
    let mut axioms: expert_sys::Set = expert_sys::Set::default();
    axioms.set_imply('b', 'a');
    axioms.set_imply('c', 'b');

    let not_not_a: expert_sys::ops::Not = expert_sys::ops::Not::new (
        std::rc::Rc::new(expert_sys::ops::Not::new (
            axioms.get_exp('a').unwrap(),
        )),
    );
    let not_b: expert_sys::ops::Not = expert_sys::ops::Not::new (
        axioms.get_exp('b').unwrap(),
    );
    let not_c: expert_sys::ops::Not = expert_sys::ops::Not::new (
        axioms.get_exp('c').unwrap(),
    );
    assert_eq!(not_not_a.get_value(), Some(false));
    assert_eq!(not_b.get_value(), Some(true));
    assert_eq!(not_c.get_value(), Some(true));
}
