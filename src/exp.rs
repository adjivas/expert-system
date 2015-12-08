// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Exp` Trait is a expression implemented.
/// for: axiom, not, and, or, xor.

pub trait Exp <'a> {
    fn get_value(&'a self) -> bool;
    fn get_ident(&'a self) -> String;
}
