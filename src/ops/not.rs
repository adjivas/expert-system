// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use ops::{Exp, ExpPtr};
use std::rc::Rc;
use std::cell::RefCell;
use ops::{Set};

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

    fn get_value(&self, initial_values: &Set) -> bool {
        match self.infer.borrow().get_value(initial_values) {
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

    fn set_value(&self, set: &mut Set, new_value: bool) {
        self.infer.borrow().set_value(set, !new_value);
    }

    fn list_axiom(&self) -> Vec<char> {
        self.infer.borrow().list_axiom()
    }

    fn replace_axiom(&mut self, axiom: char, replacement: ExpPtr) {
        let is_axiom = self.infer.borrow().is_axiom(axiom);
        match is_axiom {
            true => self.infer = replacement.clone(),
            false => self.infer.borrow_mut().
                    replace_axiom(axiom, replacement.clone()),
        }
    }
}
