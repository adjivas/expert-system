// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use exp::Exp;
use set::Set;

pub struct Solver {
    tree: [Vec<std::rc::Rc<Exp>>; 26],
}

impl Solver {
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

    pub fn get_branch_value (
        &mut self,
        index: char,
    ) -> Option<bool> {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(grade) = std::rc::Rc::downgrade (
                    &self.tree[i - 65].last().unwrap()
                ).upgrade() {
                    grade.get_value()
                }
                else { None }
            },
            i @ 97...122 => {
                if let Some(grade) = std::rc::Rc::downgrade(
                    &self.tree[i - 97].last().unwrap()
                ).upgrade() {
                    grade.get_value()
                }
                else { None }
            },
            _ => None,
        }
    }

    /// The `get_ident` function returns the axiom's expression.

    fn get_branch_ident (&self, index: char) -> Option<String> {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(axiom) = std::rc::Rc::downgrade (
                    &self.tree[i - 65].last().unwrap()
                ).upgrade() {
                    axiom.get_ident()
                }
                else { None }
            },
            i @ 97...122 => {
                if let Some(axiom) = std::rc::Rc::downgrade (
                    &self.tree[i - 97].last().unwrap()
                ).upgrade() {
                    axiom.get_ident()
                }
                else { None }
            },
            _ => None,
        }
    }

    fn get_branch_exp (&self, index: char) -> Option<std::rc::Rc<Exp>> {
        match {index as usize} {
            i @ 65...90 => {
                if let Some(grade) = std::rc::Rc::downgrade (
                    &self.tree[i - 65].last().unwrap()
                ).upgrade() {
                    Some(grade)
                }
                else { None }
            },
            i @ 97...122 => {
                if let Some(grade) = std::rc::Rc::downgrade(
                    &self.tree[i - 97].last().unwrap()
                ).upgrade() {
                    Some(grade)
                }
                else { None }
            },
            _ => None,
        }
    }

    fn get_branch_exps (&self, index: char) -> Option<Vec<std::rc::Rc<Exp>>> {
        match {index as usize} {
            t @ 65...90 => {
                Some(self.tree[t - 65].iter().filter_map (|ref b|
                    if let Some(grade) = std::rc::Rc::downgrade (
                        &b
                    ).upgrade() {
                        Some(grade)
                    } else { None }
                ).collect::<Vec<std::rc::Rc<Exp>>>())
            },
            t @ 97...122 => {
                Some(self.tree[t - 97].iter().filter_map (|ref b|
                    if let Some(grade) = std::rc::Rc::downgrade (
                        &b
                    ).upgrade() {
                        Some(grade)
                    } else { None }
                ).collect::<Vec<std::rc::Rc<Exp>>>())
            },
            _ => None,
        }
    }

    pub fn set_branch_imply (
        &mut self,
        top: char,
        bottom: std::rc::Rc<Exp>,
    ) -> bool {
        match {top as usize} {
            t @ 65...90 => {
                self.tree[t - 65].push(bottom);
                true
            },
            t @ 97...122 => {
                self.tree[t - 97].push(bottom);
                true
            },
            _ => false,
        }
    }
}

macro_rules! format_exp {
    ($exp: expr) => {
        match $exp {
            Some(axioms) => {
                axioms.iter().map(|ref g|
                    format!("{}=>", g)
                ).collect::<String>() + match axioms.last() {
                    Some(axiom) if axiom.get_value() == Some(true) => "true",
                    Some(axiom) if axiom.get_value() == Some(false) => "false",
                    _ => "None",
                }
            },
            _ => format!("None=>None"),
        }
    };
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
