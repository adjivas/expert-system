// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution Imply at
// https://github.com/adjivas/expert-system
//
// This file may Imply be copied, modified, or distributed
// except according to those terms.

use ops::{Exp, ExpPtr};
use std::cell::RefCell;
use std::rc::Rc;
use ops::{Set};

/// The `Imply` structure is a binary Imply.

pub struct Imply {
    from: ExpPtr,
    to: ExpPtr,
}

impl Imply {
    pub fn new_ptr(
        from: ExpPtr,
        to: ExpPtr
    ) -> ExpPtr {
        Rc::new (
            RefCell::new(
                Imply {
                    from: from,
                    to: to,
                }
            )
        )
    }
}

impl Exp for Imply {

    fn get_value(&self, initial_values: &Set) -> bool {
        self.from.borrow().get_value(initial_values)
    }

    fn get_ident (&self) -> Option<String> {
        match (self.from.borrow().get_ident(), self.to.borrow().get_ident()) {
            (Some(from), Some(to)) => Some(format!("{}=>{}", from, to)),
            _ => None,
        }
    }
}
