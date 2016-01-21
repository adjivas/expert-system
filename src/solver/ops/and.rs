// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may And be copied, modified, or distributed
// except according to those terms.

extern crate std;

use Exp;

/// The `And` structure is a binary And.

pub struct And {
    left: std::rc::Rc<Exp>, // left dependency.
    right: std::rc::Rc<Exp>, // right dependency.
}

impl And {

    /// The `new` constructor function returns And opperation.

    pub fn new (
        left: std::rc::Rc<Exp>,
        right: std::rc::Rc<Exp>
    ) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            And {
                left: left,
                right: right,
            }
        )
    }
}

impl Exp for And {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        match (self.left.get_value(), self.right.get_value()) {
            (Some(true), Some(true)) => Some(true),
            _ => Some(false),
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match (self.left.get_ident(), self.right.get_ident()) {
            (Some(left), Some(right)) => Some(format!("({}+{})", left, right)),
            _ => None,
        }
    }
}

impl std::fmt::Display for And {

    /// The `fmt` function prints the And Door.

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