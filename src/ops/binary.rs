// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `Binary` Trait is a expression implemented.
/// for: and, or, xor.

use exp::Exp;

pub trait Binary : Exp {
    fn new (left: *mut Exp, right: *mut Exp) -> Self;
}
