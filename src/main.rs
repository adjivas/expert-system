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
    *b = true;
    let a_and_b = expert_sys::ops::And::new (
        &mut a as *mut expert_sys::Axiom,
        &mut b as *mut expert_sys::Axiom
    );

    let a_and_b__and__a_and_b = expert_sys::ops::And::new (
        &a_and_b as *const expert_sys::ops::And<*mut expert_sys::Axiom>,
        &a_and_b as *const expert_sys::ops::And<*mut expert_sys::Axiom>
    );

    println!("{}", a_and_b__and__a_and_b);
    /*let mut b: expert_sys::Axiom = expert_sys::Axiom::new('b');
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new_imply('a', &mut b as *mut expert_sys::Axiom);
    let not_a = expert_sys::ops::Not::new(&mut a as *mut expert_sys::Axiom);
    let not_not_a = expert_sys::ops::Not::new(&not_a as *const expert_sys::ops::Not<*mut expert_sys::Axiom>);

    println!("{:?}", not_a);
    println!("{:?}", not_not_a);*/
    /*let mut b: expert_sys::Axiom = expert_sys::Axiom::new_imply('b', &mut a as *mut expert_sys::Axiom);
    let c: expert_sys::Axiom = expert_sys::Axiom::new_imply('c', &mut b as *mut expert_sys::Axiom);

    *a = true;
    println!("{}", c == c);
    println!("{}", c);*/
}
