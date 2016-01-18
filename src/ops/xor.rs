// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution Xor at
// https://github.com/adjivas/expert-system
//
// This file may Xor be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::Binary;
use Exp;

/// The `Xor` structure is a binary Xor.

pub struct Xor {
    left: std::rc::Rc<Exp>, // left dependency.
    right: std::rc::Rc<Exp>, // right dependency.
}

impl Binary for Xor {

    /// The `new` constructor function returns Xor opperation.

    fn new (
        left: std::rc::Rc<Exp>,
        right: std::rc::Rc<Exp>
    ) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            Xor {
                left: left,
                right: right,
            }
        )
    }
}

impl Exp for Xor {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        match (self.left.get_value(), self.right.get_value()) {
            (Some(true), Some(false)) => Some(true),
            (Some(false), Some(true)) => Some(true),
            _ => Some(false),
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match (self.left.get_ident(), self.right.get_ident()) {
            (Some(left), Some(right)) => Some(format!("({}^{})", left, right)),
            _ => None,
        }
    }
}

impl std::fmt::Display for Xor {

    /// The `fmt` function prints the Xor Door.

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
