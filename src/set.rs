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

    fn get_value (&self, index: char) -> Option<bool> {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(grade) = std::rc::Rc::downgrade (
                    &self.axioms[i - 65]
                ).upgrade() {
                    grade.get_value()
                }
                else { None }
            },
            i @ 97...122 => {
                if let Some(grade) = std::rc::Rc::downgrade(
                    &self.axioms[i - 97]
                ).upgrade() {
                    grade.get_value()
                }
                else { None }
            },
            _ => None,
        }
    }

    /// The `get_ident` function returns the axiom's expression.

    fn get_ident (&self, index: char) -> Option<String> {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(axiom) = std::rc::Rc::downgrade (
                    &self.axioms[i - 65]
                ).upgrade() {
                    axiom.get_ident()
                }
                else { None }
            },
            i @ 97...122 => {
                if let Some(axiom) = std::rc::Rc::downgrade (
                    &self.axioms[i - 97]
                ).upgrade() {
                    axiom.get_ident()
                }
                else { None }
            },
            _ => None,
        }
    }

    pub fn get_exp (&self, index: char) -> Option<std::rc::Rc<Exp>> {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(grade) = std::rc::Rc::downgrade (
                    &self.axioms[i - 65]
                ).upgrade() {
                    Some(grade)
                }
                else { None }
            },
            i @ 97...122 => {
                if let Some(grade) = std::rc::Rc::downgrade(
                    &self.axioms[i - 97]
                ).upgrade() {
                    Some(grade)
                }
                else { None }
            },
            _ => None,
        }
    }

    /// The `set_value` function updates the axiom's boolean value.

    pub fn set_value (
        &mut self,
        index: char,
        value: bool
    ) -> bool {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(axiom) = std::rc::Rc::get_mut (
                    &mut self.axioms[i - 65]
                ) {
                    axiom.set_value(value);
                    true
                }
                else { false }
            },
            i @ 97...122 => {
                if let Some(axiom) = std::rc::Rc::get_mut (
                    &mut self.axioms[i - 97]
                ) {
                    axiom.set_value(value);
                    true
                }
                else { false }
            },
            _ => false,
        }
    }

    pub fn set_imply (&mut self, index_left: char, index_right: char) -> bool {
        match ({index_left as usize}, {index_right as usize}) {
            (l @ 65...90, r @ 65...90) => {
                if let Some(grade) = std::rc::Rc::downgrade (
                    &self.axioms[r - 65]
                ).upgrade() {
                    if let Some(axiom) = std::rc::Rc::get_mut (
                        &mut self.axioms[l - 65]
                    ) {
                        axiom.set_imply(grade);
                        true
                    }
                    else {
                        false
                    }
                }
                else { false }
            },
            (l @ 97...122, r @ 97...122) => {
                if let Some(grade) = std::rc::Rc::downgrade (
                    &self.axioms[r - 97]
                ).upgrade() {
                    if let Some(axiom) = std::rc::Rc::get_mut (
                        &mut self.axioms[l - 97]
                    ) {
                        axiom.set_imply(grade);
                        true
                    }
                    else {
                        false
                    }
                }
                else { false }
            },
            _ => false,
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
            i @ 65...90 => &self.axioms[i - 65],
            i @ 97...122 => &self.axioms[i - 97],
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
