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
mod set;
mod axiom;
pub mod exp;
#[cfg(test)]
mod test_exp;

pub use self::and::And;
pub use self::not::Not;
pub use self::xor::Xor;
pub use self::or::Or;
pub use self::imply::{Imply, ImplyPtr};
pub use self::set::{Set};
pub use self::axiom::{Axiom, AxiomPtr};
pub use self::exp::{Exp, ExpPtr};

fn merge_axiom_vector(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
	let mut to_return = (*v1).clone();
    for c1 in v1 {
        for c2 in v2 {
            if *c1 == *c2 {
                continue ;
            }
            to_return.push(*c2);
        }
    }
    to_return
}
