// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

fn main () {
    let mut b: expert_sys::Axiom = expert_sys::Axiom::new('b');
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');

    *a = true;
    a.set_imply(&mut b as *mut expert_sys::Axiom);
    *b = true;
    let a_and_b = expert_sys::ops::And::new (
        &mut a as *mut expert_sys::Axiom,
        &mut b as *mut expert_sys::Axiom
    );

    println!("{}", a_and_b);
}
