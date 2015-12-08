// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Exp;
use expert_sys::ops::Unary;

fn main () {
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');
    let mut b: expert_sys::Axiom = expert_sys::Axiom::new('b');
    let mut not_a = expert_sys::ops::Not::new(&mut a);
    let not_b = expert_sys::ops::Not::new(&mut b);

    *a = true;
    not_a.set_imply(&mut b);
    println!("{}=>{}", not_a.get_ident(), not_a.get_value());
    println!("{}=>{}", not_b.get_ident(), not_b.get_value());
}
