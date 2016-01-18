// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The `ops` module is a collection of operators
//! like: Axiom, And, Or, Xor, Not, [...].

mod and;
mod not;
mod xor;
mod or;
mod imply;

pub use self::and::And;
pub use self::not::Not;
pub use self::xor::Xor;
pub use self::or::Or;
pub use self::imply::Imply;
