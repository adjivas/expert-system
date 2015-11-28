// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Fact` structure is a binary axiom.

#[derive(Debug, PartialEq)]
pub struct Fact {
    pub ident: String, // logical expression.
    pub value: bool, // result.
}

impl Fact {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (ident: String) -> Self {
        Fact {
            ident: ident,
            value: false,
        }
    }

    /// The `new_rev` constructor function returns a true axiom.

    pub fn new_rev (ident: String) -> Self {
        Fact {
            ident: ident,
            value: true,
        }
    }
}

impl std::ops::Add for Fact {
    type Output = Fact;

    /// The `add` function returns axiom and axiom.

    fn add(self, other: Fact) -> Fact {
        Fact {
            ident: self.ident + &other.ident,
            value: self.value && other.value,
        }
    }
}

impl std::ops::BitOr for Fact {
    type Output = Fact;

    /// The `bitor` function returns axiom or axiom.

    fn bitor(self, other: Fact) -> Fact {
        Fact {
            ident: self.ident + "|" + &other.ident,
            value: self.value || other.value,
        }
    }
}

impl std::ops::BitXor for Fact {
    type Output = Fact;

    /// The `bitxor` function returns axiom xor axiom.

    fn bitxor(self, other: Fact) -> Fact {
        Fact {
            ident: self.ident + "^" + &other.ident,
            value: self.value ^ other.value,
        }
    }
}

impl std::ops::Not for Fact {
    type Output = Fact;

    /// The `not` function returns not axiom.

    fn not(self) -> Fact {
        Fact {
            ident: "!".to_string() + &self.ident,
            value: !self.value,
        }
    }
}

impl std::fmt::Display for Fact {

    /// The `fmt` function prints the axiom.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
		write!(f, "{} => {}", self.ident, self.value)
	}
}

impl Default for Fact {

    /// The `default` constructor function returns a false axiom.

    fn default() -> Self {
		Fact::new("_".to_string())
    }
}
