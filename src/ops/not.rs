// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::Unary;
use exp::Exp;

/// The `Not` structure is a binary Not.

pub struct Not {
    infer: std::rc::Rc<Exp>, // infer dependencies.
    imply: Option<std::rc::Rc<Exp>>, // implication.
}

impl Unary for Not {

    /// The `new` constructor function returns Not opperation.

    fn new (infer: std::rc::Rc<Exp>) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            Not {
                infer: infer,
                imply: None,
            }
        )
    }
}

impl Exp for Not {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        match self.imply {
            Some(ref imply) => imply.get_value(),
            None if self.infer.get_value() == Some(true) => Some(false),
            None if self.infer.get_value() == Some(false) => Some(true),
            _ => None,
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match self.imply {
            Some(ref imply) => {
                if let (Some(infer), Some(imply)) = (
                    self.infer.get_ident(),
                    imply.get_ident()
                ) {
                    Some(format! ("(!{}=>{})", infer, imply))
                }
                else {
                    None
                }
            },
            None => if let Some(infer) = self.infer.get_ident() {
                Some(format! ("!({})", infer))
            }
            else {
                None
            },
        }
    }

    /// The `set_imply` function changes the Not implication.

    fn set_imply (&mut self, imply: std::rc::Rc<Exp>) -> bool {
        self.imply = Some(imply);
        true
    }
}

impl std::fmt::Display for Not {

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
