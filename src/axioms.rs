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

pub struct Axioms<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm,
            'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> (
    pub Axiom<'a>, pub Axiom<'b>, pub Axiom<'c>, pub Axiom<'d>, pub Axiom<'e>,
    pub Axiom<'f>, pub Axiom<'g>, pub Axiom<'h>, pub Axiom<'i>, pub Axiom<'j>,
    pub Axiom<'k>, pub Axiom<'l>, pub Axiom<'m>, pub Axiom<'n>, pub Axiom<'o>,
    pub Axiom<'p>, pub Axiom<'q>, pub Axiom<'r>, pub Axiom<'s>, pub Axiom<'t>,
    pub Axiom<'u>, pub Axiom<'v>, pub Axiom<'w>, pub Axiom<'x>, pub Axiom<'y>,
    pub Axiom<'z>
);

impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> Set for Axioms
     <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {

    /// The `new` constructor function returns the axiom set.

    fn new () -> Self {
        Axioms (
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

/*pub enum Value<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n,
      'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    a(Axiom<'a>), b(Axiom<'b>), c(Axiom<'c>), d(Axiom<'d>),
    e(Axiom<'e>), f(Axiom<'f>), g(Axiom<'g>), h(Axiom<'h>),
    i(Axiom<'i>), j(Axiom<'j>), k(Axiom<'k>), l(Axiom<'l>),
    m(Axiom<'m>), n(Axiom<'n>), o(Axiom<'o>), p(Axiom<'p>),
    q(Axiom<'q>), r(Axiom<'r>), s(Axiom<'s>), t(Axiom<'t>),
    u(Axiom<'u>), v(Axiom<'v>), w(Axiom<'w>), x(Axiom<'x>),
    y(Axiom<'y>), z(Axiom<'z>)
}

impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n,
      'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> std::ops::Index<usize> for Axioms
      <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
       'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    type Output = Value<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n,
          'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z>;

    fn index(&self, index: usize) -> &Value<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n,
          'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
        match index {
            0 => &Value::a(self.0),
            _ => &Value::b(self.1),
        }
    }
}*/
/*
impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n,
      'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> std::ops::IndexMut<usize> for Axioms
      <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
       'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {
    fn index_mut<'_>(&mut self, index: usize) -> &mut Axiom<'_> {
        match index {
            0 => &mut self.0,
            _ => &mut self.1,
        }
    }
}*/

impl <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> Default for Axioms
     <'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h,
      'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x, 'y, 'z> {

    /// The `default` constructor function returns the axiom set.

    fn default () -> Self {
        Axioms::new ()
    }
}
