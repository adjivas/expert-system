// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::rc::Rc;
use std::cell::RefCell;
use ops::{Set};

pub type ExpPtr = Rc<RefCell<Exp>>;

/// The `Exp` Trait is an expression's implemention.
pub trait Exp {

	/// Return a string describing the expression (like the `Display` trait).
    fn get_ident(&self) -> Option<String>;

    /// return the value of this expression
    fn get_value(&self, initial_values: &Set) -> bool;

    /// Set the values implied by this expression
    fn set_value(&self, set: &mut Set, new_value: bool);

    /// Return the list of all the axiom in this expression
    fn list_axiom(&self) -> Vec<char>;

    /// Replace axiom of index `axiom` by the expression `override`
    fn replace_axiom(&mut self, axiom: char, replacement: ExpPtr);

    fn is_axiom(&self, index: char) -> bool {
        false
    }

    fn eq(&self, other: Rc<Exp>) -> bool {
        self.get_ident() == other.get_ident()
    }

    /// Return true if the axiom is in this expression
    fn axiom_is_present(&self, to_search: char) -> bool {
        let axioms = self.list_axiom();
        for a in axioms {
            if a == to_search {
                return true;
            }
        }
        false
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Exp
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        write!(f, "{}", self.get_ident().unwrap());
        Ok(())
    }
}
