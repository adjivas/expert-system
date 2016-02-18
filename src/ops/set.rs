// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use ops::{Axiom, AxiomPtr, Exp};
use std::rc::Rc;
use std::collections::HashMap;
use std::ops::Index;

/// The `Set` structure is all alphabet axioms.
pub struct Set {
    axioms: HashMap<char, bool>,
}

impl Set {

    pub fn new() -> Set {
        let mut axioms = HashMap::with_capacity(26);
        axioms.insert('A', false);
        axioms.insert('B', false);
        axioms.insert('C', false);
        axioms.insert('D', false);
        axioms.insert('E', false);
        axioms.insert('F', false);
        axioms.insert('G', false);
        axioms.insert('H', false);
        axioms.insert('I', false);
        axioms.insert('J', false);
        axioms.insert('K', false);
        axioms.insert('L', false);
        axioms.insert('M', false);
        axioms.insert('N', false);
        axioms.insert('O', false);
        axioms.insert('P', false);
        axioms.insert('Q', false);
        axioms.insert('R', false);
        axioms.insert('S', false);
        axioms.insert('T', false);
        axioms.insert('U', false);
        axioms.insert('V', false);
        axioms.insert('W', false);
        axioms.insert('X', false);
        axioms.insert('Y', false);
        axioms.insert('Z', false);
        Set {
            axioms: axioms
        }
    }

    /// The `get_value` function returns the axiom's boolean.
    pub fn get_value (&self, index: char) -> bool {
        self.axioms[&index]
    }

    /// The `set_value` function updates the axiom's boolean value.
    pub fn set_value (&mut self, index: char, new_value: bool) {
        let keyval = self.axioms.entry(index).or_insert(false);
        *keyval = new_value;
    }

    /// Set `value` for axiom `index`, index being a string of size 1.
    pub fn set_value_str(&mut self, index: &str, value: bool) {
        let c = index.chars().next().unwrap();
        self.set_value(c, value);
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Set {

    /// The `fmt` function prints all axioms.
    fn fmt (
        &self,
        f: &mut Formatter,
    ) -> Result<(), Error> {
        write! (f, "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},\
                    {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            self.get_value('a'), self.get_value('b'), self.get_value('c'), self.get_value('d'), self.get_value('e'), self.get_value('f'),
            self.get_value('g'), self.get_value('h'), self.get_value('i'), self.get_value('j'), self.get_value('k'), self.get_value('l'),
            self.get_value('m'), self.get_value('n'), self.get_value('o'), self.get_value('p'), self.get_value('q'), self.get_value('r'),
            self.get_value('s'), self.get_value('t'), self.get_value('u'), self.get_value('v'), self.get_value('w'), self.get_value('x'),
            self.get_value('y'), self.get_value('z'),
        )
    }
}
