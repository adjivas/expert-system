// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `format_exp` macro returns the String of all dependencies
/// from a expression.

#[macro_export]
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
