// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::Set;
use axiom::Axiom;

/// The `Axioms` type is the collection of all axioms.

pub type Axioms<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> = (
    Axiom<'a>, Axiom<'b>, Axiom<'c>, Axiom<'d>, Axiom<'e>, Axiom<'f>, Axiom<'g>,
    Axiom<'h>, Axiom<'i>, Axiom<'j>, Axiom<'k>, Axiom<'l>, Axiom<'m>, Axiom<'n>,
    Axiom<'o>, Axiom<'p>, Axiom<'q>, Axiom<'r>, Axiom<'s>, Axiom<'t>, Axiom<'u>,
    Axiom<'v>, Axiom<'w>, Axiom<'x>, Axiom<'y>, Axiom<'z>
);

impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> Set for Axioms <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {

    /// The `new` constructor function returns the axiom set.

    fn new () -> Self {
        (
            Axiom::new('a'), Axiom::new('b'), Axiom::new('c'), Axiom::new('d'),
            Axiom::new('e'), Axiom::new('f'), Axiom::new('g'), Axiom::new('h'),
            Axiom::new('i'), Axiom::new('j'), Axiom::new('k'), Axiom::new('l'),
            Axiom::new('m'), Axiom::new('n'), Axiom::new('o'), Axiom::new('p'),
            Axiom::new('q'), Axiom::new('r'), Axiom::new('s'), Axiom::new('t'),
            Axiom::new('u'), Axiom::new('v'), Axiom::new('w'), Axiom::new('x'),
            Axiom::new('y'), Axiom::new('z')
        )
    }
}
