// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;

pub struct Set <'a> {
    axioms: [Axiom<'a>; 26],
}

impl <'a> std::ops::Index<char> for Set<'a> {
    type Output = Axiom<'a>;

    /// The `index` function returns the axiom from set.

    fn index (&self, index: char) -> &Axiom<'a> {
        match {index as usize} {
            i @ 65...90 => &self.axioms[i - 65],
            i @ 97...122 => &self.axioms[i - 97],
            _ => unimplemented!(),
        }
    }
}

impl <'a> std::ops::IndexMut<char> for Set<'a> {

    /// The `index_mut` function returns a mutable axiom from set.

    fn index_mut (&mut self, index: char) -> &mut Axiom<'a> {
        match {index as usize} {
            i @ 65...90 => &mut self.axioms[i - 65],
            i @ 97...122 => &mut self.axioms[i - 97],
            _ => unimplemented!(),
        }
    }
}

impl <'a> Default for Set <'a> {

    /// The `default` constructor function returns the axiom set.

    fn default () -> Self {
        Set {
            axioms: [
                Axiom::new('a'), Axiom::new('b'), Axiom::new('c'),
                Axiom::new('d'), Axiom::new('e'), Axiom::new('f'),
                Axiom::new('g'), Axiom::new('h'), Axiom::new('i'),
                Axiom::new('j'), Axiom::new('k'), Axiom::new('l'),
                Axiom::new('m'), Axiom::new('n'), Axiom::new('o'),
                Axiom::new('p'), Axiom::new('q'), Axiom::new('r'),
                Axiom::new('s'), Axiom::new('t'), Axiom::new('u'),
                Axiom::new('v'), Axiom::new('w'), Axiom::new('x'),
                Axiom::new('y'), Axiom::new('z')
            ]
        }
    }
}
