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
    ident: char, // name.
    value: bool, // result.
    imply: Option<std::rc::Rc<Axiom>>, // implication.
}

impl Axiom {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (ident: char) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            Axiom {
                ident: ident,
                value: false,
                imply: None,
            }
        )
    }

    /// The `set_value` function updates the axiom's value.

    pub fn set_value (
        &mut self,
        value: bool,
    ) -> bool {
        self.value = value;
        true
    }

    /// The `set_imply` function adds/updates the axiom's implication.

    pub fn set_imply (
        &mut self,
        imply: std::rc::Rc<Axiom>,
    ) {
        self.imply = Some(imply);
    }
}

impl Exp for Axiom {

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        match self.imply {
            Some(ref imply) => {
                if let Some(grade) = std::rc::Rc::downgrade(imply).upgrade() {
                    grade.get_value()
                }
                else { None }
            },
            None => Some(self.value),
        }
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        match self.imply {
            Some(ref imply) => {
                if let Some(grade) = std::rc::Rc::downgrade(imply).upgrade() {
                    if let Some(result) = grade.get_ident() {
                        Some(format!("{}=>{}", self.ident, result))
                    }
                    else { None }
                }
                else { None }
            },
            None => Some(format!("{}", self.ident)),
        }
    }

    /// The `set_imply` function changes the Axiom implication.

    fn set_imply (&mut self, _: std::rc::Rc<Exp>) -> bool {
        false
    }
}

impl std::fmt::Display for Axiom {

    /// The `fmt` function prints the Axiom.

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
