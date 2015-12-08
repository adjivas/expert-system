// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

#[allow(unused_variables)]
fn main () {
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');
    let not_a = expert_sys::ops::Not::new (
        &mut a as *mut expert_sys::Axiom,
    );
    //println!("{}", not_a);
}
