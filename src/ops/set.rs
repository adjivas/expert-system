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
#[derive(Debug, PartialEq)]
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

    /// Return a set where every value corresponding to each key of the chars
    /// in the `s` string set to true.
    #[cfg(test)]
    pub fn from_str(s: &str) -> Set {
        let mut to_return = Set::new();
        for c in s.chars() {
            to_return.set_value(c, true);
        }
        to_return
    }

    /// The `get_value` function returns the axiom's boolean.
    pub fn get_value(&self, index: char) -> bool {
        self.axioms[&index]
    }

    /// The `set_value` function updates the axiom's boolean value.
    pub fn set_value(&mut self, index: char, new_value: bool) {
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
        write! (f, "A({}), B({}), C({}), D({}), E({}), F({}), G({}), H({}), I({}), \
                    J({}), K({}), L({}), M({}), N({}), O({}), P({}), Q({}), R({}), \
                    S({}), T({}), U({}), V({}), W({}), X({}), Y({}), Z({})",
            self.get_value('A'), self.get_value('B'), self.get_value('C'), self.get_value('D'), self.get_value('E'), self.get_value('F'),
            self.get_value('G'), self.get_value('H'), self.get_value('I'), self.get_value('J'), self.get_value('K'), self.get_value('L'),
            self.get_value('M'), self.get_value('N'), self.get_value('O'), self.get_value('P'), self.get_value('Q'), self.get_value('R'),
            self.get_value('S'), self.get_value('T'), self.get_value('U'), self.get_value('V'), self.get_value('W'), self.get_value('X'),
            self.get_value('Y'), self.get_value('Z'),
        )
    }
}
