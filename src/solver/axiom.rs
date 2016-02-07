// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use Exp;

/// The `Axiom` structure is a primitive.

pub struct Axiom {
    ident: char, // name.
    value: bool, // result.
}

impl Axiom {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (ident: char) -> std::rc::Rc<Self> {
        std::rc::Rc::new (
            Axiom {
                ident: match ident as u8 {
                    l @ 65u8...90u8 => l,
                    l @ 97u8...122u8 =>l-32u8,
                    _ => unimplemented!(),
                } as char,
                value: false,
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
}

impl Exp for Axiom {

    // The `put_eval_imply` function returns the value and prints the implication.

    fn put_eval_imply (
        &self,
        rules: &Vec<std::rc::Rc<Exp>>
    ) -> bool {
        let result: bool = self.value;
        let a: Option<String> = self.get_ident();

        for rule in rules.iter() {
            if let Some(b) = std::rc::Rc::downgrade(&rule).upgrade() {
                if a == b.get_ident_right() {
                    if let Some(expr) = b.get_exprs_left() {
                        result = expr.put_eval_imply(&rules);
                        break ;
                    }
                }
            }
        }
        result
    }

    /// The `get_value` function returns the result.

    fn get_value (&self) -> Option<bool> {
        Some(self.value)
    }

    /// The `get_ident` function returns the arithmetic formule.

    fn get_ident (&self) -> Option<String> {
        Some(format!("{}", self.ident))
    }

    /// The `get_ident_left` function returns nothing.

    fn get_ident_left (&self) -> Option<String> {
        None
    }


    /// The `get_ident_right` function returns nothing.

    fn get_ident_right (&self) -> Option<String> {
        None
    }

    /// The `get_exprs_left` function returns nothing.

    fn get_exprs_left (&self) -> Option<std::rc::Rc<Exp>> {
        None
    }

    /// The `get_exprs_right` function returns tnothing.

    fn get_exprs_right (&self) -> Option<std::rc::Rc<Exp>> {
        None
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
