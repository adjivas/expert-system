// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution Or at
// https://github.com/adjivas/expert-system
//
// This file may Or be copied, modified, or distributed
// except according to those terms.

use ops::{Exp, ExpPtr};
use std::cell::RefCell;
use std::rc::Rc;
use ops::{Set};
use ops;

/// The `Or` structure is a binary Or.
pub struct Or {
    left: ExpPtr, // left dependency.
    right: ExpPtr, // right dependency.
}

impl Or {

    pub fn new (
        left: ExpPtr,
        right: ExpPtr
    ) -> ExpPtr {
        Rc::new (
            RefCell::new(
                Or {
                    left: left,
                    right: right,
                }
            )
        )
    }
}

impl Exp for Or {

    fn get_value(&self, initial_values: &Set) -> bool {
        match (
            self.left.borrow().get_value(initial_values),
            self.right.borrow().get_value(initial_values)
        ) {
            (true, false) => true,
            (false, true) => true,
            (true, true) => true,
            _ => false,
        }
    }

    fn get_ident (&self) -> Option<String> {
        match (self.left.borrow().get_ident(), self.right.borrow().get_ident()) {
            (Some(left), Some(right)) => Some(format!("({}|{})", left, right)),
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
