// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

fn main () {
    let mut a = expert_sys::Axiom::new('a');
    let mut b = expert_sys::Axiom::new('b');

    a.set_imply(&mut b);
    *b = true;
    println!("{}", a);
}
