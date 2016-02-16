// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `parse_index` macro returns a formated Index or Nothing.

#[macro_use]
macro_rules! parse_index {
    ($index: expr) => {
        match {$index as usize} {
            i @ 0...25 => Some(i),
            i @ 65...90 => Some(i-65),
            i @ 97...122 => Some(i-97),
            _ => None,
        }
    };
    ($left: expr, $right: expr) => {
        match ($left as usize, $right as usize) {
            (l @ 0...25, r @ 0...25) => Some((l, r)),
            (l @ 65...90, r @ 65...90) => Some((l-65, r-65)),
            (l @ 97...122, r @ 97...122) => Some((l-97, r-97)),
            _ => None,
        }
    };
}

/// The `format_exp` macro returns the String of all dependencies
/// from a expression.
#[macro_use]
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
            _ => format!("None"),
        }
    };
}
