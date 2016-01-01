// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use exp::Exp;
use set::Set;

/// The `Solver` structure is all alphabet expression tree.

pub struct Solver {
    tree: [Vec<std::rc::Rc<Exp>>; 26],
}

impl Solver {

    /// The `new` constructor function returns the tree solver.

    pub fn new (
        axioms: &Set,
    ) -> Self {
        Solver {
            tree: [
                vec!(axioms.get_exp('a').unwrap()),
                vec!(axioms.get_exp('b').unwrap()),
                vec!(axioms.get_exp('c').unwrap()),
                vec!(axioms.get_exp('d').unwrap()),
                vec!(axioms.get_exp('e').unwrap()),
                vec!(axioms.get_exp('f').unwrap()),
                vec!(axioms.get_exp('g').unwrap()),
                vec!(axioms.get_exp('h').unwrap()),
                vec!(axioms.get_exp('i').unwrap()),
                vec!(axioms.get_exp('j').unwrap()),
                vec!(axioms.get_exp('k').unwrap()),
                vec!(axioms.get_exp('l').unwrap()),
                vec!(axioms.get_exp('m').unwrap()),
                vec!(axioms.get_exp('n').unwrap()),
                vec!(axioms.get_exp('o').unwrap()),
                vec!(axioms.get_exp('p').unwrap()),
                vec!(axioms.get_exp('q').unwrap()),
                vec!(axioms.get_exp('r').unwrap()),
                vec!(axioms.get_exp('s').unwrap()),
                vec!(axioms.get_exp('t').unwrap()),
                vec!(axioms.get_exp('u').unwrap()),
                vec!(axioms.get_exp('v').unwrap()),
                vec!(axioms.get_exp('w').unwrap()),
                vec!(axioms.get_exp('x').unwrap()),
                vec!(axioms.get_exp('y').unwrap()),
                vec!(axioms.get_exp('z').unwrap()),
            ]
        }
    }

    /// The `get_branch_value` function returns the axiom's value.

    pub fn get_branch_value (
        &mut self,
        index: char,
    ) -> Option<bool> {
        if let Some(i) = parse_index!(index) {
            if let Some(grade) = std::rc::Rc::downgrade (
                &self.tree[i].last().unwrap()
            ).upgrade() {
                grade.get_value()
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// The `get_ident` function returns the axiom's expression.

    fn get_branch_ident (
        &self,
        index: char,
    ) -> Option<String> {
        if let Some(i) = parse_index!(index) {
            if let Some(axiom) = std::rc::Rc::downgrade (
                &self.tree[i].last().unwrap()
            ).upgrade() {
                axiom.get_ident()
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// The `get_branch_exp` function returns the last axiom's branch.

    pub fn get_branch_exp (
        &self,
        index: char,
    ) -> Option<std::rc::Rc<Exp>> {
        if let Some(i) = parse_index!(index) {
            if let Some(grade) = std::rc::Rc::downgrade (
                &self.tree[i].last().unwrap()
            ).upgrade() {
                Some(grade)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// The `get_branch_exp` function returns the all axiom' branch.

    fn get_branch_exps (
        &self,
        index: char,
    ) -> Option<Vec<std::rc::Rc<Exp>>> {
        if let Some(t) = parse_index!(index) {
            Some(self.tree[t].iter().filter_map (|ref b|
                if let Some(grade) = std::rc::Rc::downgrade (
                    &b
                ).upgrade() {
                    Some(grade)
                } else { None }
            ).collect::<Vec<std::rc::Rc<Exp>>>())
        }
        else {
            None
        }
    }

    /// The `set_branch_imply` function pushs the expression to
    /// the targeted branch.

    fn set_branch_imply (
        &mut self,
        top: char,
        bottom: std::rc::Rc<Exp>,
    ) -> bool {
        if let Some(t) = parse_index!(top) {
            self.tree[t].push(bottom);
            true
        }
        else {
            false
        }
    }

    /// The `add_branch_exp` function pushs the expression
    /// to the last targeted branch.

    pub fn add_branch_exp (
        &mut self,
        top: std::rc::Rc<Exp>,
        bottom: String,
    ) -> bool {
        if let Some(index) = {
            (0u8..25u8).zip (
                self.tree.iter()
            ).scan(bottom, |ident, (index, branch)| {
                match branch.last() {
                    Some(ref axiom) if axiom.get_ident().is_some()
                                    && axiom.get_ident().unwrap() == *ident => {
                        Some((index)-1)
                    },
                    _ => None,
                }
            }).collect::<Vec<u8>>().last()
        } {
            self.set_branch_imply((*index) as char, top)
        }
        else {
            false
        }
    }
}

impl std::fmt::Display for Solver {

    /// The `fmt` function prints all the tree dependencies.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
    write! (f, "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, \
                {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            format_exp!(self.get_branch_exps('a')),
            format_exp!(self.get_branch_exps('b')),
            format_exp!(self.get_branch_exps('c')),
            format_exp!(self.get_branch_exps('d')),
            format_exp!(self.get_branch_exps('e')),
            format_exp!(self.get_branch_exps('f')),
            format_exp!(self.get_branch_exps('g')),
            format_exp!(self.get_branch_exps('h')),
            format_exp!(self.get_branch_exps('i')),
            format_exp!(self.get_branch_exps('j')),
            format_exp!(self.get_branch_exps('k')),
            format_exp!(self.get_branch_exps('l')),
            format_exp!(self.get_branch_exps('m')),
            format_exp!(self.get_branch_exps('n')),
            format_exp!(self.get_branch_exps('o')),
            format_exp!(self.get_branch_exps('p')),
            format_exp!(self.get_branch_exps('q')),
            format_exp!(self.get_branch_exps('r')),
            format_exp!(self.get_branch_exps('s')),
            format_exp!(self.get_branch_exps('t')),
            format_exp!(self.get_branch_exps('u')),
            format_exp!(self.get_branch_exps('v')),
            format_exp!(self.get_branch_exps('w')),
            format_exp!(self.get_branch_exps('x')),
            format_exp!(self.get_branch_exps('y')),
            format_exp!(self.get_branch_exps('z')),
        )
    }
}
