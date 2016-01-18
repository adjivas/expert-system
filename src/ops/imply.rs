// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution Imply at
// https://github.com/adjivas/expert-system
//
// This file may Imply be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::Binary;
use Exp;

/// The `Imply` structure is a binary Imply.

pub struct Imply {
    from: std::rc::Rc<Exp>,
    to: std::rc::Rc<Exp>,
}

impl Binary for Imply {

    /// The `new` constructor function returns Imply opperation.

    fn new (
        from: std::rc::Rc<Exp>,
        to:: std::rc::Rc<Exp>
    ) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            Imply {
                from: from,
                to: to:,
            }
        )
    }
}

impl Exp for Imply {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        self.to.get_value()
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match (self.from.get_ident(), self.to.get_ident()) {
            (Some(from), Some(to)) => Some(format!("({}=>{})", from, to)),
            _ => None,
        }
    }
}

impl std::fmt::Display for Imply {

    /// The `fmt` function prints the Imply Door.

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
