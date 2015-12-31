// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may And be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::Binary;
use Exp;

/// The `And` structure is a binary And.

pub struct And {
    left: std::rc::Rc<Exp>, // left dependency.
    right: std::rc::Rc<Exp>, // right dependency.
    imply: Option<std::rc::Rc<Exp>>, // implication.
}

impl Binary for And {

    /// The `new` constructor function returns And opperation.

    fn new (left: std::rc::Rc<Exp>, right: std::rc::Rc<Exp>) -> Self {
        And {
            left: left,
            right: right,
            imply: None,
        }
    }
}

impl Exp for And {

    /// The `set_imply` function changes the And implication.

    fn set_imply (&mut self, imply: std::rc::Rc<Exp>) {
        self.imply = Some(imply);
    }

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        match self.imply {
            Some(ref imply) => imply.get_value(),
            None if self.left.get_value() == None => {
                None
            },
            None if self.left.get_value() == Some(true)
                 && self.right.get_value() == Some(true) => {
                Some(true)
            },
            _ => Some(false),
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match self.imply {
            Some(ref imply) => {
                if let (Some(left), Some(right), Some(imply)) = (
                    self.left.get_ident(),
                    self.right.get_ident(),
                    imply.get_ident(),
                ) {
                    Some(format!("({}+{}=>{})", left, right, imply))
                }
                else {
                    None
                }
            },
            None => if let (Some(left), Some(right)) = (
                self.left.get_ident(),
                self.right.get_ident(),
            ) {
                Some(format!("({}+{})", left, right))
            }
            else {
                None
            },
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
