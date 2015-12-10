// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Set;

fn main () {
    let mut axioms: expert_sys::Set = Set::default();
    let solver = expert_sys::Branch::new (
        &mut axioms.a,
        &mut axioms.b,
        &mut axioms.c,
        &mut axioms.d,
        &mut axioms.e,
        &mut axioms.f,
        &mut axioms.g,
        &mut axioms.h,
        &mut axioms.i,
        &mut axioms.j,
        &mut axioms.k,
        &mut axioms.l,
        &mut axioms.m,
        &mut axioms.n,
        &mut axioms.o,
        &mut axioms.p,
        &mut axioms.q,
        &mut axioms.r,
        &mut axioms.s,
        &mut axioms.t,
        &mut axioms.u,
        &mut axioms.v,
        &mut axioms.w,
        &mut axioms.x,
        &mut axioms.y,
        &mut axioms.z,
    );
}
