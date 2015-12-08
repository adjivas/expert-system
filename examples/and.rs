// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Exp;
use expert_sys::ops::Binary;

fn main () {
    let mut a = expert_sys::Axiom::new('a');
    let mut b = expert_sys::Axiom::new('b');
    let mut c = expert_sys::Axiom::new('c');

    a.set_imply(&mut b);
    *b = true;
    let mut a_and_b = expert_sys::ops::And::new (
        &mut a,
        &mut b
    );
    let mut b_and_c = expert_sys::ops::And::new (
        &mut b,
        &mut c
    );

    b_and_c.set_imply(&mut a_and_b);
    println!("{} {}", a_and_b.get_ident(), a_and_b.get_value());
    println!("{} {}", b_and_c.get_ident(), b_and_c.get_value());
}
