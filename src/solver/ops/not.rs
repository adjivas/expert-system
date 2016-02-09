// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use Exp;
use Rules;

/// The `Not` structure is a binary Not.

pub struct Not {
    infer: std::rc::Rc<Exp>, // infer dependencies.
}

impl  Not {

    /// The `new` constructor function returns Not opperation.

    pub fn new (infer: std::rc::Rc<Exp>) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            Not {
                infer: infer,
            }
        )
    }
}

impl Exp for Not {

    // The `put_eval_imply` function returns the value and prints the implication.

    fn put_eval_imply (
        &self,
        rules: &Rules,
    ) -> Option<bool> {
        match rules.get_imply (&self.infer) {
            Some(true) => Some(false),
            Some(false) => Some(true),
            None => None,
        }
    }

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        match self.infer.get_value() {
            Some(true) => Some(false),
            Some(false) => Some(true),
            None => None,
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match self.infer.get_ident() {
            Some(infer) => Some(format! ("!({})", infer)),
            None => None,
        }
    }

    /// The `get_ident_left` function returns nothing.

    fn get_ident_left (&self) -> Option<String> {
        None
    }


    /// The `get_ident_right` function returns nothing.

    fn get_ident_right (&self) -> Option<String> {
        None
    }

    /// The `get_exprs_left` function returns the nothing.

    fn get_exprs_left (&self) -> Option<std::rc::Rc<Exp>> {
        None
    }

    /// The `get_exprs_right` function returns the nothing.

    fn get_exprs_right (&self) -> Option<std::rc::Rc<Exp>> {
        None
    }
}

impl  std::fmt::Display for Not {

    /// The `fmt` function prints the Not.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        match (self.get_ident(), self.get_value()) {
            (Some(ident), Some(value)) => write!(f, "{}=>{}", ident, value),
            (_, _) => write!(f, "None"),
        }
    }
}
