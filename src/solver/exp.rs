// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use std::rc::Rc;

/// The `Exp` Trait is an expression's implemention.

pub trait Exp {
    fn get_ident(&self) -> Option<String>;
    fn get_value(&self) -> Option<bool>;

    fn eq(&self, other: Rc<Exp>) -> bool {
        self.get_ident() == other.get_ident()
    }
}

impl std::fmt::Display for Exp {

    /// The `fmt` function prints a generic expression.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        match (self.get_ident(), self.get_value()) {
            (Some(ident), Some(value)) => write!(f, "({}=>{})", ident, value),
            (_, _) => write!(f, "None"),
        }
    }
}