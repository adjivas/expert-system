// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;
use exp::Exp;
use std::rc::Rc;

/// The `Set` structure is all alphabet axioms.

pub struct Set {
    axioms: [Rc<Axiom>; 26],
}

impl Set {

    pub fn new() -> Set {
        Set {
            axioms: [
                Axiom::new('A'),
                Axiom::new('B'),
                Axiom::new('C'),
                Axiom::new('D'),
                Axiom::new('E'),
                Axiom::new('F'),
                Axiom::new('G'),
                Axiom::new('H'),
                Axiom::new('I'),
                Axiom::new('J'),
                Axiom::new('K'),
                Axiom::new('L'),
                Axiom::new('M'),
                Axiom::new('N'),
                Axiom::new('O'),
                Axiom::new('P'),
                Axiom::new('Q'),
                Axiom::new('R'),
                Axiom::new('S'),
                Axiom::new('T'),
                Axiom::new('U'),
                Axiom::new('V'),
                Axiom::new('W'),
                Axiom::new('X'),
                Axiom::new('Y'),
                Axiom::new('Z'),
            ]
        }
    }

    /// The `get_value` function returns the axiom's boolean.

    pub fn get_value (
        &self,
        index: char,
    ) -> Option<bool> {
        if let Some(i) = parse_index!(index) {
            if let Some(grade) = Rc::downgrade (
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

    pub fn get_ident (
        &self,
        index: char,
    ) -> Option<String> {
        if let Some(i) = parse_index!(index) {
            if let Some(axiom) = Rc::downgrade (
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
    ) -> Option<Rc<Exp>> {
        if let Some(i) = parse_index!(index) {
            if let Some(grade) = Rc::downgrade (
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
        value: bool,
    ) -> bool {
        if let Some(i) = parse_index!(index) {
            match Rc::get_mut (
                &mut self.axioms[i]
            ) {
                Some(axiom) => axiom.set_value(value),
                None => false,
            }
        }
        else {
            false
        }
    }

    pub fn set_value_str(&mut self, index: &str, value: bool) -> bool {
        let c = index.chars().next().unwrap();
        self.set_value(c, value)
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

    fn index (
        &self,
        index: char,
    ) -> &Axiom {
        match parse_index!(index) {
            Some(i) => &self.axioms[i],
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
