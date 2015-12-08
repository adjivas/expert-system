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

pub struct And<'a, 'b, 'c> {
    left: *mut Exp<'a>, // left dependency.
    right: *mut Exp<'b>, // right dependency.
    imply: Option<*mut Exp<'c>>, // implication.
}

impl <'a, 'b, 'c> Binary<'a, 'b, 'c> for And<'a, 'b, 'c> {

    /// The `new` constructor function returns And opperation.

    fn new (left: *mut Exp<'a>, right: *mut Exp<'b>) -> Self {
        And {
            left: left,
            right: right,
            imply: None,
        }
    }

    /// The `set_imply` function changes the And implication.

    fn set_imply<'e> (&'e mut self, imply: *mut Exp<'c>) {
        self.imply = Some(imply);
    }
}

impl <'a, 'b, 'c> Exp<'c> for And<'a, 'b, 'c> {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> bool {
        match self.imply {
            Some(imply) => unsafe { &*imply }.get_value(),
            None => {
                unsafe { &*self.left }.get_value() &&
                unsafe { &*self.right }.get_value()
            },
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> String {
        match self.imply {
            Some(imply) => format! ("({}+{}=>{})",
                &unsafe { &*self.left }.get_ident(),
                &unsafe { &*self.right }.get_ident(),
                &unsafe { &*imply }.get_ident(),
            ),
            None => format! ("({}+{})",
                &unsafe { &*self.left }.get_ident(),
                &unsafe { &*self.right }.get_ident(),
            ),
        }
    }
}

impl <'a, 'b, 'c> std::fmt::Display for And<'a, 'b, 'c> {

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

impl <'a, 'b, 'c> std::fmt::Debug for And<'a, 'b, 'c> {

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
