// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;
use exp::Exp;

/// The `Set` structure is all alphabet axioms.

pub struct Set {
    axioms: [std::rc::Rc<Axiom>; 26],
}

impl Set {

    /// The `get_value` function returns the axiom's boolean.

    pub fn get_value (
        &self,
        index: char,
    ) -> Option<bool> {
        if let Some(i) = match {index as usize} {
            i @ 0...25 => Some(i),
            i @ 65...90 => Some(i-65),
            i @ 97...122 => Some(i-97),
            _ => None,
        } {
            if let Some(grade) = std::rc::Rc::downgrade (
                &self.axioms[i]
            ).upgrade() {
                grade.get_value()
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// The `get_ident` function returns the axiom's name.

    pub fn get_ident (&self, index: char) -> Option<String> {
        if let Some(i) = match {index as usize} {
            i @ 0...25 => Some(i),
            i @ 65...90 => Some(i-65),
            i @ 97...122 => Some(i-97),
            _ => None,
        } {
            if let Some(axiom) = std::rc::Rc::downgrade (
                &self.axioms[i]
            ).upgrade() {
                axiom.get_ident()
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// The `get_exp` function returns the expression.

    pub fn get_exp (
        &self,
        index: char
    ) -> Option<std::rc::Rc<Exp>> {
        if let Some(i) = match {index as usize} {
            i @ 0...25 => Some(i),
            i @ 65...90 => Some(i-65),
            i @ 97...122 => Some(i-97),
            _ => None,
        } {
            if let Some(grade) = std::rc::Rc::downgrade (
                &self.axioms[i]
            ).upgrade() {
                Some(grade)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// The `set_value` function updates the axiom's boolean value.

    pub fn set_value (
        &mut self,
        index: char,
        value: bool
    ) -> bool {
        if let Some(i) = match {index as usize} {
            i @ 0...25 => Some(i),
            i @ 65...90 => Some(i-65),
            i @ 97...122 => Some(i-97),
            _ => None,
        } {
            if let Some(axiom) = std::rc::Rc::get_mut (
                &mut self.axioms[i]
            ) {
                axiom.set_value(value);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// The `set_imply` function inserts/updates the dependency axiom.

    pub fn set_imply (
        &mut self,
        left: char,
        right: char
    ) -> bool {
        if let Some((l, r)) = match (left as usize, right as usize) {
            (l @ 0...25, r @ 0...25) => Some((l, r)),
            (l @ 65...90, r @ 65...90) => Some((l-65, r-65)),
            (l @ 97...122, r @ 97...122) => Some((l-97, r-97)),
            _ => None,
        } {
            if let Some(grade) = std::rc::Rc::downgrade (
                &self.axioms[r]
            ).upgrade() {
                if let Some(axiom) = std::rc::Rc::get_mut (
                    &mut self.axioms[l]
                ) {
                    axiom.set_imply(grade);
                    true
                }
                else {
                    false
                }
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
}

impl Default for Set {

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
                Axiom::new('y'), Axiom::new('z'),
            ]
        }
    }
}

impl std::ops::Index<char> for Set {
    type Output = Axiom;

    /// The `index` function returns the axiom from set.

    fn index (&self, index: char) -> &Axiom {
        match {index as usize} {
            i @ 0...25 => &self.axioms[i],
            i @ 65...90 => &self.axioms[i-65],
            i @ 97...122 => &self.axioms[i-97],
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Set {

    /// The `fmt` function prints all axioms.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write! (f, "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},\
                    {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            self['a'], self['b'], self['c'], self['d'], self['e'], self['f'],
            self['g'], self['h'], self['i'], self['j'], self['k'], self['l'],
            self['m'], self['n'], self['o'], self['p'], self['q'], self['r'],
            self['s'], self['t'], self['u'], self['v'], self['w'], self['x'],
            self['y'], self['z'],
        )
    }
}
