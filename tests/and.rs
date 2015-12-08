// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

#[test]
fn test_value () {
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');
    let mut b: expert_sys::Axiom = expert_sys::Axiom::new('b');
    /*let a_and_b: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut a as *mut expert_sys::Axiom,
        &mut b as *mut expert_sys::Axiom
    );

    a.set_imply(&mut b as *mut expert_sys::Axiom);
    *b = true;
    //assert_eq!(a_and_b.get_value(), true);*/
}
