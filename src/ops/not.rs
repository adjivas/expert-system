// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use ops::{Exp, ExpPtr};
use std::rc::Rc;
use std::cell::RefCell;

/// The `Not` structure is a binary Not.
pub struct Not {
    infer: ExpPtr,
}

impl Not {
    pub fn new_ptr(infer: ExpPtr) -> ExpPtr {
        Rc::new (
            RefCell::new(
                Not {
                    infer: infer,
                }
            )
        )
    }
}

impl Exp for Not {

    fn get_value (&self) -> bool {
        match self.infer.borrow().get_value() {
            true => false,
            false => true,
        }
    }

    fn get_ident (&self) -> Option<String> {
        match self.infer.borrow().get_ident() {
            Some(infer) => Some(format! ("!({})", infer)),
            None => None,
        }
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Not {

    /// The `fmt` function prints the Not.

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
