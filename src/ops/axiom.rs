// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use ops::Exp;
use std::rc::Rc;
use std::cell::RefCell;

pub type AxiomPtr = Rc<RefCell<Axiom>>;

/// The `Axiom` structure is a primitive.
pub struct Axiom {
    ident: char, // name.
    value: bool, // result.
}

impl Axiom {

    /// The `new` constructor function returns a default false axiom.
    pub fn new_ptr(ident: char) -> AxiomPtr {
        Rc::new (
            RefCell::new(
                Axiom {
                    ident: ident,
                    value: false,
                }
            )
        )
    }

    /// The `set_value` function updates the axiom's value.
    pub fn set_value (
        &mut self,
        value: bool,
    ) -> bool {
        self.value = value;
        true
    }
}

impl Exp for Axiom {

    fn get_value (&self) -> bool {
        self.value
    }

    fn get_ident (&self) -> Option<String> {
        Some(format!("{}", self.ident))
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Axiom {

    /// The `fmt` function prints the Axiom.
    fn fmt (
        &self,
        f: &mut Formatter,
    ) -> Result<(), Error> {
        match (self.get_ident(), self.get_value()) {
            (Some(ident), value) => write!(f, "{}=>{}", ident, value),
            (_, _) => write!(f, "None"),
        }

    }
}
