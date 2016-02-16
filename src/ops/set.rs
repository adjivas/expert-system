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
    axioms: HashMap<char, AxiomPtr>,
}

impl Set {

    pub fn new() -> Set {
        let mut axioms = HashMap::with_capacity(26);
        axioms.insert('A', Axiom::new_ptr('A'));
        axioms.insert('B', Axiom::new_ptr('B'));
        axioms.insert('C', Axiom::new_ptr('C'));
        axioms.insert('D', Axiom::new_ptr('D'));
        axioms.insert('E', Axiom::new_ptr('E'));
        axioms.insert('F', Axiom::new_ptr('F'));
        axioms.insert('G', Axiom::new_ptr('G'));
        axioms.insert('H', Axiom::new_ptr('H'));
        axioms.insert('I', Axiom::new_ptr('I'));
        axioms.insert('J', Axiom::new_ptr('J'));
        axioms.insert('K', Axiom::new_ptr('K'));
        axioms.insert('L', Axiom::new_ptr('L'));
        axioms.insert('M', Axiom::new_ptr('M'));
        axioms.insert('N', Axiom::new_ptr('N'));
        axioms.insert('O', Axiom::new_ptr('O'));
        axioms.insert('P', Axiom::new_ptr('P'));
        axioms.insert('Q', Axiom::new_ptr('Q'));
        axioms.insert('R', Axiom::new_ptr('R'));
        axioms.insert('S', Axiom::new_ptr('S'));
        axioms.insert('T', Axiom::new_ptr('T'));
        axioms.insert('U', Axiom::new_ptr('U'));
        axioms.insert('V', Axiom::new_ptr('V'));
        axioms.insert('W', Axiom::new_ptr('W'));
        axioms.insert('X', Axiom::new_ptr('X'));
        axioms.insert('Y', Axiom::new_ptr('Y'));
        axioms.insert('Z', Axiom::new_ptr('Z'));
        Set {
            axioms: axioms
        }
    }

    /// The `get_value` function returns the axiom's boolean.
    pub fn get_value (&self, index: char) -> bool {
        self.axioms[&index].borrow().get_value()
    }

    /// The `set_value` function updates the axiom's boolean value.
    pub fn set_value (&mut self, index: char, value: bool) {
        let axiom = self.axioms[&index].borrow_mut().set_value(value);
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
