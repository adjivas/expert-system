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
use ops;

pub type ImplyPtr = Rc<RefCell<Imply>>;

/// The `Imply` structure is a binary Imply.
pub struct Imply {
    pub from: ExpPtr,
    to: ExpPtr,
}

impl Imply {
    pub fn new_ptr(
        from: ExpPtr,
        to: ExpPtr
    ) -> ImplyPtr {
        Rc::new (
            RefCell::new(
                Imply {
                    from: from,
                    to: to,
                }
            )
        )
    }

    /// Change the value stored in result_values according to this
    /// expression if necessary.
    pub fn solve(&self,
        initial_facts: &Set,
        result_values: &mut Set
    ) -> bool {
        let new_value = self.from.borrow().get_value(initial_facts);
        self.set_value(result_values, new_value);
        true
    }

    /// Return true if this instruction change the value of the `axiom`
    pub fn imply_axiom(&self, axiom: char) -> bool {
        self.to.borrow().axiom_is_present(axiom)
    }
}

impl Exp for Imply {

    fn get_value(&self, initial_facts: &Set) -> bool {
        self.from.borrow().get_value(initial_facts)
    }

    fn get_ident (&self) -> Option<String> {
        match (self.from.borrow().get_ident(), self.to.borrow().get_ident()) {
            (Some(from), Some(to)) => Some(format!("{}=>{}", from, to)),
            _ => None,
        }
    }

    fn set_value(&self, set: &mut Set, new_value: bool) {
        self.to.borrow().set_value(set, new_value);
    }

    fn list_axiom(&self) -> Vec<char> {
        let v1 = self.from.borrow().list_axiom();
        let v2 = self.to.borrow().list_axiom();
        ops::merge_axiom_vector(&v1, &v2)
    }

    /// Warning, here, only replace axiom on the from branch !
    fn replace_axiom(&mut self, axiom: char, replacement: ExpPtr) {
        let is_axiom = self.from.borrow().is_axiom(axiom);
        match is_axiom {
            true => self.from = replacement.clone(),
            false => self.from.borrow_mut().
                    replace_axiom(axiom, replacement.clone()),
        }
    }
}
