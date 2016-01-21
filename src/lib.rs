// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod macros;
mod solver;
pub mod command;

pub use solver::ops;
pub use solver::Exp;
pub use solver::Set;
pub use solver::Axiom;
