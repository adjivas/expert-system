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
    infer: Box<Exp>, // infer dependencies.
    imply: Option<Box<Exp>>, // implication.
}

impl Unary for Not {

    /// The `new` constructor function returns Not opperation.

    fn new (infer: *mut Exp) -> Self {
        Not {
            infer: unsafe { Box::from_raw(infer) },
            imply: None,
        }
    }
}

impl Exp for Not {

    /// The `set_imply` function changes the And implication.

    fn set_imply (&mut self, imply: *mut Exp) {
        self.imply = Some (
            unsafe { Box::from_raw(imply) },
        );
    }

    /// The `get_value` function returns the result.

    fn get_value (&self) -> bool {
        match self.imply {
            Some(ref imply) => imply.get_value(),
            None => !self.infer.get_value(),
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> String {
        match self.imply {
            Some(ref imply) => format! ("(!{}=>{})",
                self.infer.get_ident(),
                imply.get_ident(),
            ),
            None => format! ("!{}",
                self.infer.get_ident(),
            ),
        }
    }
}

impl std::fmt::Display for Not {

    /// The `fmt` function prints the Not.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}=>{}", self.get_ident(), self.get_value())
    }
}

impl std::fmt::Debug for Not {

    /// The `fmt` function prints the Not.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}=>{:?}", self.get_ident(), self.get_value())
    }
}
