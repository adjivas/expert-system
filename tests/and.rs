// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.
/*
extern crate expert_sys;

use expert_sys::Exp;
use expert_sys::ops::Binary;
use expert_sys::ops::Unary;

#[test]
fn test_value () {
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');
    let mut b: expert_sys::Axiom = expert_sys::Axiom::new('b');
    let a_and_b: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut a as *mut expert_sys::Axiom,
        &mut b as *mut expert_sys::Axiom
    );

    a.set_imply(&mut b as *mut expert_sys::Axiom);
    *b = true;
    assert_eq!(a_and_b.get_value(), true);
}
/*
B => A
D + E => B
G + H => F
I + J => G
G => H
L + M => K
O + P => L + N
N => M
*/

#[test]
#[allow(unused_variables, unused_mut)]
fn test_correction () {
    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');
    let mut b: expert_sys::Axiom = expert_sys::Axiom::new('b');
    let mut c: expert_sys::Axiom = expert_sys::Axiom::new('c');
    let mut d: expert_sys::Axiom = expert_sys::Axiom::new('d');
    let mut e: expert_sys::Axiom = expert_sys::Axiom::new('e');
    let mut f: expert_sys::Axiom = expert_sys::Axiom::new('f');
    let mut g: expert_sys::Axiom = expert_sys::Axiom::new('g');
    let mut h: expert_sys::Axiom = expert_sys::Axiom::new('h');
    let mut i: expert_sys::Axiom = expert_sys::Axiom::new('i');
    let mut j: expert_sys::Axiom = expert_sys::Axiom::new('j');
    let mut k: expert_sys::Axiom = expert_sys::Axiom::new('k');
    let mut l: expert_sys::Axiom = expert_sys::Axiom::new('l');
    let mut m: expert_sys::Axiom = expert_sys::Axiom::new('m');
    let mut n: expert_sys::Axiom = expert_sys::Axiom::new('n');
    let mut o: expert_sys::Axiom = expert_sys::Axiom::new('o');
    let mut p: expert_sys::Axiom = expert_sys::Axiom::new('p');
    let mut d_and_e: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut d as *mut expert_sys::Axiom,
        &mut e as *mut expert_sys::Axiom
    );
    let mut g_and_h: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut g as *mut expert_sys::Axiom,
        &mut h as *mut expert_sys::Axiom
    );
    let mut i_and_j: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut i as *mut expert_sys::Axiom,
        &mut j as *mut expert_sys::Axiom
    );
    let mut l_and_m: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut l as *mut expert_sys::Axiom,
        &mut m as *mut expert_sys::Axiom
    );
    let mut l_and_n: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut l as *mut expert_sys::Axiom,
        &mut n as *mut expert_sys::Axiom
    );
    let mut o_and_p: expert_sys::ops::And = expert_sys::ops::And::new (
        &mut o as *mut expert_sys::Axiom,
        &mut p as *mut expert_sys::Axiom
    );

    b.set_imply(&mut a as *mut expert_sys::Axiom);
    g.set_imply(&mut h as *mut expert_sys::Axiom);
    n.set_imply(&mut m as *mut expert_sys::Axiom);

    d_and_e.set_imply(&mut b as *mut expert_sys::Axiom);
    g_and_h.set_imply(&mut f as *mut expert_sys::Axiom);
    i_and_j.set_imply(&mut g as *mut expert_sys::Axiom);
    l_and_m.set_imply(&mut k as *mut expert_sys::Axiom);
    o_and_p.set_imply(&mut l_and_n as *mut expert_sys::ops::And);

    *d = true;
    *e = true;
    *i = true;
    *j = true;
    *o = true;
    *p = true;
}
*/
