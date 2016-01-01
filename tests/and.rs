// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::Exp;
use expert_sys::ops::Binary;

/// The `test_example` checks the demonstration.

#[test]
fn test_example () {
    let mut axioms = expert_sys::Set::default();

    axioms.set_value('a', true);
    axioms.set_value('b', true);
    let a_and_b = expert_sys::ops::And::new (
        axioms.get_exp('a').unwrap(),
        axioms.get_exp('b').unwrap()
    );
    let b_and_c = expert_sys::ops::And::new (
        axioms.get_exp('b').unwrap(),
        axioms.get_exp('c').unwrap()
    );
    assert_eq!(a_and_b.get_value(), Some(true));
    assert_eq!(b_and_c.get_value(), Some(false));
}

/// The `test_advanced` checks this graph:
///                     G=>H              N=>M
///  A  B   C D E F      G   H I J  K L M  N   O P Q R S T U V W Y Z
///     |         |      |          |
///   D+E=>B    G+H=>F I+J=>G     L+M=>K
///                           O+P=>(L+N=>L+M)

fn test_advanced () {
    let mut axioms = expert_sys::Set::default();
    axioms.set_value('d', true);
    axioms.set_value('e', true);
    axioms.set_imply('g', 'h');
    axioms.set_imply('n', 'm');
    let mut solver = expert_sys::Solver::new(&axioms);
    let e_and_d: std::rc::Rc<expert_sys::Exp> = expert_sys::ops::And::new (
        solver.get_branch_exp('d').unwrap(),
        solver.get_branch_exp('e').unwrap()
    );
    solver.set_branch_imply(e_and_d, "b".to_string());
    assert_eq!(solver.get_branch_value('a'), Some(false));
    assert_eq!(solver.get_branch_value('b'), Some(true));
}
