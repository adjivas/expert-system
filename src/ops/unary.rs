// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `Unary` Trait is a expression implemented.
/// for: not.

use exp::Exp;

pub trait Unary<'a, 'b, 'c> : Exp<'c> {
    fn new (infer: *mut Exp<'a>) -> Self;
    fn set_imply (&mut self, imply: *mut Exp<'b>);
}
