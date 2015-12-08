// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use exp::Exp;

/// The `Not` structure is a binary Not.

pub struct Not<'a, 'b>  {
    infer: *mut Exp<'a>, // infer dependencies.
    imply: Option<*mut Exp<'b>>, // implication.
}

impl <'a, 'b> Not<'a, 'b> {

    /// The `new` constructor function returns Not opperation.

    pub fn new (infer: *mut Exp<'a>) -> Self {
        Not {
            infer: infer,
            imply: None,
        }
    }
}

impl <'a, 'b, 'c> Exp <'c> for Not<'a, 'b> {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> bool {
        match self.imply {
            Some(imply) => unsafe { &*imply }.get_value(),
            None => !unsafe { &*self.infer }.get_value(),
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> String {
        "!".to_string() +
        &unsafe { &*self.infer }.get_ident()
    }
}

impl <'a, 'b> std::fmt::Display for Not<'a, 'b> {

    /// The `fmt` function prints the Not.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}=>{}", self.get_ident(), self.get_value())
    }
}

impl <'a, 'b> std::fmt::Debug for Not<'a, 'b> {

    /// The `fmt` function prints the Not.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}=>{:?}", self.get_ident(), self.get_value())
    }
}
