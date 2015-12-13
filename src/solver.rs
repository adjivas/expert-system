// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use exp::Exp;
use set::Set;

pub struct Solver {
    branchs: [Box<Exp>; 26],
}

impl Solver {

    /// The `new` constructor function returns a solvable table.

    pub fn new (
        axioms: &mut Set,
    ) -> Self {
        Solver {
            branchs: [
                unsafe { Box::from_raw (&mut axioms['a']) },
                unsafe { Box::from_raw (&mut axioms['b']) },
                unsafe { Box::from_raw (&mut axioms['c']) },
                unsafe { Box::from_raw (&mut axioms['d']) },
                unsafe { Box::from_raw (&mut axioms['e']) },
                unsafe { Box::from_raw (&mut axioms['f']) },
                unsafe { Box::from_raw (&mut axioms['g']) },
                unsafe { Box::from_raw (&mut axioms['h']) },
                unsafe { Box::from_raw (&mut axioms['i']) },
                unsafe { Box::from_raw (&mut axioms['j']) },
                unsafe { Box::from_raw (&mut axioms['k']) },
                unsafe { Box::from_raw (&mut axioms['l']) },
                unsafe { Box::from_raw (&mut axioms['m']) },
                unsafe { Box::from_raw (&mut axioms['n']) },
                unsafe { Box::from_raw (&mut axioms['o']) },
                unsafe { Box::from_raw (&mut axioms['p']) },
                unsafe { Box::from_raw (&mut axioms['q']) },
                unsafe { Box::from_raw (&mut axioms['r']) },
                unsafe { Box::from_raw (&mut axioms['s']) },
                unsafe { Box::from_raw (&mut axioms['t']) },
                unsafe { Box::from_raw (&mut axioms['u']) },
                unsafe { Box::from_raw (&mut axioms['v']) },
                unsafe { Box::from_raw (&mut axioms['w']) },
                unsafe { Box::from_raw (&mut axioms['x']) },
                unsafe { Box::from_raw (&mut axioms['y']) },
                unsafe { Box::from_raw (&mut axioms['z']) },
            ],
        }
    }
}
