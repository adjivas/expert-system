// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use ops::Exp;
use std::rc::Rc;
use std::cell::RefCell;
use ops::{Set};

pub type AxiomPtr = Rc<RefCell<Axiom>>;

/// The `Axiom` structure is a primitive.
pub struct Axiom {
    ident: char, // name.
}

impl Axiom {

    /// The `new` constructor function returns a default false axiom.
    pub fn new_ptr(ident: char) -> AxiomPtr {
        Rc::new (
            RefCell::new(
                Axiom {
                    ident: ident,
                }
            )
        )
    }

    /// The `set_value` function updates the axiom's value.
    pub fn set_value (
        &mut self,
        result_values: &mut Set,
        new_value: bool
    ) {
        result_values.set_value(self.ident, new_value);
    }
}

impl Exp for Axiom {

    fn get_value(&self, initial_values: &Set) -> bool {
        initial_values.get_value(self.ident)
    }

    fn get_ident(&self) -> Option<String> {
        Some(format!("{}", self.ident))
    }

    fn set_value(&self, set: &mut Set, new_value: bool) {
        set.set_value(self.ident, new_value);
    }
}
