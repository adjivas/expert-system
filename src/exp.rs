// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `Exp` Trait is a expression implemented.
/// for the axiom Structure, Unary and Binary Traits.

pub trait Exp {
    fn get_value (&self) -> bool;
    fn get_ident (&self) -> String;
    fn set_imply (&mut self, imply: *mut Exp);
}

/*impl PartialEq for Exp {

    /// The `eq` function returns a boolean for our axiom equal another axiom.

    fn eq(&self, other: &Exp) -> bool {
        self.get_ident() == other.get_ident()
    }
}
*/
