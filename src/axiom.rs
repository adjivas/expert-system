// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use exp::Exp;

/// The `Axiom` structure is a primitive.

pub struct Axiom<'a> {
    ident: char, // logical expression.
    value: bool, // result.
    imply: Option<*mut Axiom<'a>>, // implication.
}

impl <'a> Axiom<'a> {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (ident: char) -> Self {
        Axiom {
            ident: ident,
            value: false,
            imply: None,
        }
    }

    /// The `set_imply` function changes the axiom implication.

    pub fn set_imply<'b> (&'b mut self, imply: *mut Axiom<'a>) {
        self.imply = Some(imply);
    }
}

impl <'a> Exp<'a> for Axiom<'a> {

    /// The `set_imply` function changes the axiom implication.

    fn set_imply<'d> (&'d mut self, imply: *mut Exp<'a>) {
        self.set_imply(imply as *mut Axiom);
    }

    /// The `get_value` function returns the result.

    fn get_value (&self) -> bool {
        match self.imply {
            Some(imply) => unsafe { &*imply }.get_value(),
            None => self.value,
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> String {
        match self.imply {
            Some(imply) => format! ("({}=>{})",
                self.ident,
                &unsafe { &*imply }.get_ident(),
            ),
            None => format! ("{}",
                self.ident,
            ),
        }
    }
}

impl <'a> Default for Axiom<'a> {

    /// The `default` constructor function returns a false axiom.

    fn default () -> Self {
        Axiom {
            ident: '_',
            value: false,
            imply: None,
        }
    }
}

impl <'a> std::ops::Deref for Axiom<'a> {
    type Target = bool;

    /// The `deref` function returns the axiom value.

    fn deref(&self) -> &bool {
        match self.imply {
            Some(imply) => unsafe { &**imply },
            None => &self.value,
        }
    }
}

impl <'a> std::ops::DerefMut for Axiom<'a> {

    /// The `deref` function returns a mutable reference to axion value.

    fn deref_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl <'a> PartialEq for Axiom<'a> {

    /// The `eq` function returns a boolean for our axiom equal another axiom.

    fn eq(&self, other: &Axiom<'a>) -> bool {
        self.ident == other.ident &&
        self.value == other.value &&
        match (self.imply, other.imply) {
            (Some(imply), Some(other)) if unsafe { *imply == *other } => true,
            (None, None) => true,
            (_, _) => false,
        }
    }
}

impl <'a> std::fmt::Display for Axiom<'a> {

    /// The `fmt` function prints the Axiom.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}=>{}", self.get_ident(), self.get_value())
    }
}

impl <'a> std::fmt::Debug for Axiom<'a> {

    /// The `fmt` function prints the Axiom.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}=>{:?}", self.get_ident(), self.get_value())
    }
}
