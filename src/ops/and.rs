// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may And be copied, modified, or distributed
// except according to those terms.

use ops::{Exp, ExpPtr};
use std::rc::Rc;
use std::cell::RefCell;
use ops::{Set};
use ops;

/// The `And` structure is a binary And.
pub struct And {
    left: ExpPtr, // left dependency.
    right: ExpPtr, // right dependency.
}

impl And {

    pub fn new (
        left: ExpPtr,
        right: ExpPtr
    ) -> ExpPtr {
        Rc::new (
            RefCell::new(
                And {
                    left: left,
                    right: right,
                }
            )
        )
    }
}

impl Exp for And {

    fn get_value(&self, initial_values: &Set) -> bool {
        match (
            self.left.borrow().get_value(initial_values),
            self.right.borrow().get_value(initial_values)
        ) {
            (true, true) => true,
            _ => false,
        }
    }

    fn get_ident (&self) -> Option<String> {
        match (self.left.borrow().get_ident(), self.right.borrow().get_ident()) {
            (Some(left), Some(right)) => Some(format!("({}+{})", left, right)),
            _ => None,
        }
    }

    fn set_value(&self, set: &mut Set, new_value: bool) {
        self.left.borrow().set_value(set, new_value);
        self.right.borrow().set_value(set, new_value);
    }

    fn list_axiom(&self) -> Vec<char> {
        let v1 = self.left.borrow().list_axiom();
        let v2 = self.right.borrow().list_axiom();
        ops::merge_axiom_vector(&v1, &v2)
    }
}
