// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;

/// The `Set` type is the collection of all Set.

pub struct Set<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    pub a: Axiom<'a>, pub b: Axiom<'b>, pub c: Axiom<'c>, pub d: Axiom<'d>, pub e: Axiom<'e>,
    pub f: Axiom<'f>, pub g: Axiom<'g>, pub h: Axiom<'h>, pub i: Axiom<'i>, pub j: Axiom<'j>,
    pub k: Axiom<'k>, pub l: Axiom<'l>, pub m: Axiom<'m>, pub n: Axiom<'n>, pub o: Axiom<'o>,
    pub p: Axiom<'p>, pub q: Axiom<'q>, pub r: Axiom<'r>, pub s: Axiom<'s>, pub t: Axiom<'t>,
    pub u: Axiom<'u>, pub v: Axiom<'v>, pub w: Axiom<'w>, pub x: Axiom<'x>, pub y: Axiom<'y>,
    pub z: Axiom<'z>
}

impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> Default for Set
     <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {

    /// The `default` constructor function returns the axiom set.

    fn default () -> Self {
        Set {
            a: Axiom::new('a'), b: Axiom::new('b'), c: Axiom::new('c'), d: Axiom::new('d'),
            e: Axiom::new('e'), f: Axiom::new('f'), g: Axiom::new('g'), h: Axiom::new('h'),
            i: Axiom::new('i'), j: Axiom::new('j'), k: Axiom::new('k'), l: Axiom::new('l'),
            m: Axiom::new('m'), n: Axiom::new('n'), o: Axiom::new('o'), p: Axiom::new('p'),
            q: Axiom::new('q'), r: Axiom::new('r'), s: Axiom::new('s'), t: Axiom::new('t'),
            u: Axiom::new('u'), v: Axiom::new('v'), w: Axiom::new('w'), x: Axiom::new('x'),
            y: Axiom::new('y'), z: Axiom::new('z')
        }
    }
}
