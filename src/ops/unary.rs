// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Unary` Trait is a expression implemented.
/// for: not.

use exp::Exp;

pub trait Unary: Exp {
    fn new (infer: std::rc::Rc<Exp>) -> std::rc::Rc<Self>;
}