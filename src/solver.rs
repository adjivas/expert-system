// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use exp::Exp;
use set::Set;

pub struct Solver <'a> {
    branchs: [*mut Exp<'a>; 26],
}

impl <'a> Solver<'a> {

    /// The `new` constructor function returns a solvable table.

    pub fn new (
        axioms: &mut Set<'a>,
    ) /*-> Self*/ {
        /*Solver {
            branchs: [
                &mut axioms['a'],
                &mut axioms['b'],
                &mut axioms['c'],
                &mut axioms['d'],
                &mut axioms['e'],
                &mut axioms['f'],
                &mut axioms['g'],
                &mut axioms['h'],
                &mut axioms['i'],
                &mut axioms['j'],
                &mut axioms['k'],
                &mut axioms['l'],
                &mut axioms['m'],
                &mut axioms['n'],
                &mut axioms['o'],
                &mut axioms['p'],
                &mut axioms['q'],
                &mut axioms['r'],
                &mut axioms['s'],
                &mut axioms['t'],
                &mut axioms['u'],
                &mut axioms['v'],
                &mut axioms['w'],
                &mut axioms['x'],
                &mut axioms['y'],
                &mut axioms['z']
            ],
        }*/
    }
}
