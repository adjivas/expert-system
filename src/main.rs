// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Axioms;
use expert_sys::Set;
use expert_sys::ops::Unary;
use expert_sys::ops::Binary;

fn main () {
    let mut alphabet = Axioms::new();
    let a_and_b = expert_sys::ops::And::new (
        &mut alphabet.0,
        &mut alphabet.1
    );
    println!("{}", a_and_b);
}
