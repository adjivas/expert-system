// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `Exp` Trait is a expression implemented.
/// for the axiom Structure, Unary and Binary Traits.

pub trait Exp <'a> {
    fn get_value(&self) -> bool;
    fn get_ident(&self) -> String;
    fn set_imply<'d> (&'d mut self, imply: *mut Exp<'a>);
}
