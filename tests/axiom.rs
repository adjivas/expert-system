// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

#[test]
fn test_axiom_constructor () {
    assert_eq!(expert_sys::Axiom::default(), expert_sys::Axiom::new('_'));
}

#[test]
fn test_axiom_value () {
    let mut a: expert_sys::Axiom = expert_sys::Axiom::default();

    assert_eq!(*a, false);
    *a = true;
    assert_eq!(*a, true);
}

#[test]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn test_fact_correction_and () {
    //  _             _        ___       ___        ___        ___
    //  a  b  d e     n  M     i+j  G H  G+H  f     L+M  k     o+p  L n
    //  \  \          \         \   \     \          \          \
    //  b d+e         M         G   H     f          k         L+n
    //  \                       \
    // d+e                      H
    //
    // Ownership: H, G, M, L.

    extern crate std;

    let mut a: expert_sys::Axiom = expert_sys::Axiom::new('a');
    //let b: expert_sys::Axiom = expert_sys::Axiom::new_imply('b', &mut a as *mut expert_sys::Axiom);
    let e: expert_sys::Axiom = expert_sys::Axiom::new('e');
    let d: expert_sys::Axiom = expert_sys::Axiom::new('d');
    //let d_and_e: expert_sys::Axiom = expert_sys::Axiom::new_fact(d + e);
    //let _d_and_e: expert_sys::Axiom = expert_sys::Axiom::new_fact_imply(d_and_e, std::cell::UnsafeCell::new(&b));
    /*
    let M: expert_sys::Axiom = expert_sys::Axiom::new("m".to_string());
    let n: expert_sys::Axiom = expert_sys::Axiom::new_imply("n".to_string(), vec!(std::cell::RefCell::new(&M)));

    let H: expert_sys::Axiom = expert_sys::Axiom::new("h".to_string());
    let G: expert_sys::Axiom = expert_sys::Axiom::new_imply("g".to_string(), vec!(std::cell::RefCell::new(&H)));
    let i: expert_sys::Axiom = expert_sys::Axiom::new("i".to_string());
    let j: expert_sys::Axiom = expert_sys::Axiom::new("j".to_string());
    //let i_and_j: expert_sys::Axiom = expert_sys::Axiom::new_fact_imply(i + j, vec!(std::cell::RefCell::new(&G)));
    let f: expert_sys::Axiom = expert_sys::Axiom::new_rev("f".to_string());
    //let g_and_h: expert_sys::Axiom = expert_sys::Axiom::new_fact_imply(G + H, vec!(std::cell::RefCell::new(&f)));

    let k: expert_sys::Axiom = expert_sys::Axiom::new_rev("k".to_string());
    let L: expert_sys::Axiom = expert_sys::Axiom::new("l".to_string());
    //let l_and_m: expert_sys::Axiom = expert_sys::Axiom::new_fact_imply(L + M, vec!(&k));

    let o: expert_sys::Axiom = expert_sys::Axiom::new("o".to_string());
    let p: expert_sys::Axiom = expert_sys::Axiom::new_rev("p".to_string());
    //let l_and_n: expert_sys::Axiom = expert_sys::Axiom::new_fact(L + n);
    //let o_and_p: expert_sys::Axiom = expert_sys::Axiom::new_fact(l_and_n);*/
}
