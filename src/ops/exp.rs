// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::rc::Rc;
use std::cell::RefCell;

pub type ExpPtr = Rc<RefCell<Exp>>;

/// The `Exp` Trait is an expression's implemention.
pub trait Exp {
    fn get_ident(&self) -> Option<String>;
    fn get_value(&self) -> bool;

    fn eq(&self, other: Rc<Exp>) -> bool {
        self.get_ident() == other.get_ident()
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Exp {
    fn fmt (
        &self,
        f: &mut Formatter,
    ) -> Result<(), Error> {
        match (self.get_ident(), self.get_value()) {
            (Some(ident), value) => write!(f, "({}=>{})", ident, value),
            (_, _) => write!(f, "None"),
        }
    }
}
