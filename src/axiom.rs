// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use exp::Exp;

/// The `Axiom` structure is a primitive.

pub struct Axiom {
    ident: char, // logical expression.
    value: bool, // result.
    imply: Option<Box<Axiom>>, // implication.
}

impl Axiom {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (ident: char) -> Self {
        Axiom {
            ident: ident,
            value: false,
            imply: None,
        }
    }

    /// The `set_imply` function changes the axiom implication.

    pub fn set_imply (&mut self, imply: *mut Axiom) {
        self.imply = Some (
            unsafe {
                Box::from_raw(imply)
            }
        );
    }
}

impl Exp for Axiom {

    /// The `set_imply` function changes the axiom implication.

    fn set_imply (&mut self, imply: *mut Exp) {
        self.set_imply (
            imply as *mut Axiom
        );
    }

    /// The `get_value` function returns the result.

    fn get_value (&self) -> bool {
        match self.imply {
            Some(box ref imply) => imply.get_value(),
            None => self.value,
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> String {
        match self.imply {
            Some(box ref imply) => format! ("({}=>{})",
                self.ident,
                imply.get_ident(),
            ),
            None => format! ("{}",
                self.ident,
            ),
        }
    }
}


impl Default for Axiom {

    /// The `default` constructor function returns a false axiom.

    fn default () -> Self {
        Axiom {
            ident: '_',
            value: false,
            imply: None,
        }
    }
}

impl std::ops::Deref for Axiom {
    type Target = bool;

    /// The `deref` function returns the axiom value.

    fn deref(&self) -> &bool {
        match self.imply {
            Some(box ref imply) => &*imply,
            None => &self.value,
        }
    }
}

impl std::ops::DerefMut for Axiom {

    /// The `deref` function returns a mutable reference to axion value.

    fn deref_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl PartialEq for Axiom {

    /// The `eq` function returns a boolean for our axiom equal another axiom.

    fn eq(&self, other: &Axiom) -> bool {
        self.ident == other.ident &&
        self.value == other.value &&
        self.imply == other.imply
    }
}

impl std::fmt::Display for Axiom {

    /// The `fmt` function prints the Axiom.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}=>{}", self.get_ident(), self.get_value())
    }
}

impl std::fmt::Debug for Axiom {

    /// The `fmt` function prints the Axiom.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}=>{:?}", self.get_ident(), self.get_value())
    }
}
