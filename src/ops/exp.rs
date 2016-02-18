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

    /// Change the value stored in result_values according to this
    /// expression if necessary.
    ///
    /// return `true` if the solve have been successful
    fn solve(&self, initial_values: &Set, result_values: &mut Set) -> bool {
    	true
    }

    /// Set the values implied by this expression (should only be used on
    /// imply expressions)
	///
	/// TODO is it a good idea to put this function in the `Exp` trait ?
	/// Another option is to put this function in imply and make `Rules::instrs`
	/// a vector of `ops::Imply` structs.
    fn set_value(&self, set: &mut Set, new_value: bool);

    fn eq(&self, other: Rc<Exp>) -> bool {
        self.get_ident() == other.get_ident()
    }
}
