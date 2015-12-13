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
    left: Box<Exp>, // left dependency.
    right: Box<Exp>, // right dependency.
    imply: Option<Box<Exp>>, // implication.
}

impl Binary for And {

    /// The `new` constructor function returns And opperation.

    fn new (left: *mut Exp, right: *mut Exp) -> Self {
        And {
            left: unsafe { Box::from_raw(left) },
            right: unsafe { Box::from_raw(right) },
            imply: None,
        }
    }
}


impl Exp for And {

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
            None => {
                 self.left.get_value() &&
                 self.right.get_value()
            },
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> String {
        match self.imply {
            Some(ref imply) => format! ("({}+{}=>{})",
                self.left.get_ident(),
                self.right.get_ident(),
                imply.get_ident(),
            ),
            None => format! ("({}+{})",
                self.left.get_ident(),
                self.right.get_ident(),
            ),
        }
    }
}

impl std::fmt::Display for And {

    /// The `fmt` function prints the And Door.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write! (f, "{}=>{}",
            self.get_ident(),
            self.get_value()
        )
    }
}

impl std::fmt::Debug for And {

    /// The `fmt` function prints the And Door.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write! (f, "{:?}=>{:?}",
            self.get_ident(),
            self.get_value()
        )
    }
}
