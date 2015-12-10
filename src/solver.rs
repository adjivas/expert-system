// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use exp::Exp;

pub struct Branch<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
                  'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    pub a: *mut Exp<'a>,
    pub b: *mut Exp<'b>,
    pub c: *mut Exp<'c>,
    pub d: *mut Exp<'d>,
    pub e: *mut Exp<'e>,
    pub f: *mut Exp<'f>,
    pub g: *mut Exp<'g>,
    pub h: *mut Exp<'h>,
    pub i: *mut Exp<'i>,
    pub j: *mut Exp<'j>,
    pub k: *mut Exp<'k>,
    pub l: *mut Exp<'l>,
    pub m: *mut Exp<'m>,
    pub n: *mut Exp<'n>,
    pub o: *mut Exp<'o>,
    pub p: *mut Exp<'p>,
    pub q: *mut Exp<'q>,
    pub r: *mut Exp<'r>,
    pub s: *mut Exp<'s>,
    pub t: *mut Exp<'t>,
    pub u: *mut Exp<'u>,
    pub v: *mut Exp<'v>,
    pub w: *mut Exp<'w>,
    pub x: *mut Exp<'x>,
    pub y: *mut Exp<'y>,
    pub z: *mut Exp<'z>,
}
impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> Branch<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    pub fn new (
        a: *mut Exp<'a>,
        b: *mut Exp<'b>,
        c: *mut Exp<'c>,
        d: *mut Exp<'d>,
        e: *mut Exp<'e>,
        f: *mut Exp<'f>,
        g: *mut Exp<'g>,
        h: *mut Exp<'h>,
        i: *mut Exp<'i>,
        j: *mut Exp<'j>,
        k: *mut Exp<'k>,
        l: *mut Exp<'l>,
        m: *mut Exp<'m>,
        n: *mut Exp<'n>,
        o: *mut Exp<'o>,
        p: *mut Exp<'p>,
        q: *mut Exp<'q>,
        r: *mut Exp<'r>,
        s: *mut Exp<'s>,
        t: *mut Exp<'t>,
        u: *mut Exp<'u>,
        v: *mut Exp<'v>,
        w: *mut Exp<'w>,
        x: *mut Exp<'x>,
        y: *mut Exp<'y>,
        z: *mut Exp<'z>,
    ) -> Self {
        Branch {
            a: a,
            b: b,
            c: c,
            d: d,
            e: e,
            f: f,
            g: g,
            h: h,
            i: i,
            j: j,
            k: k,
            l: l,
            m: m,
            n: n,
            o: o,
            p: p,
            q: q,
            r: r,
            s: s,
            t: t,
            u: u,
            v: v,
            w: w,
            x: x,
            y: y,
            z: z,
        }
    }
}
/*
pub struct Solver<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    branch: Branch<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
                   'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z>,
}

impl Solver<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    pub fn new (set: &mut Set) -> Self {
        Solver {
            branch: Branch::new(set),
        }
    }
}
*/
