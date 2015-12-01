// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Fact` structure is a binary axiom.

pub struct Fact<'a> {
    pub exprt: String, // logical expression.
    pub value: bool, // result.
    pub imply: Option<std::cell::UnsafeCell<&'a Fact<'a>>>, // dependencies.
}

impl <'a> Fact<'a> {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (exprt: String) -> Self {
        Fact {
            exprt: exprt,
            value: false,
            imply: None,
        }
    }

    /// The `new_imply` constructor function returns a default false axiom with dependencies.

    pub fn new_imply (exprt: String, imply: std::cell::UnsafeCell<&'a Fact<'a>>) -> Self {
        Fact {
            exprt: exprt,
            value: false,
            imply: Some(imply),
        }
    }

    pub fn new_fact (fact: Fact<'a>) -> Self {
        Fact {
            exprt: fact.exprt,
            value: fact.value,
            imply: fact.imply,
        }
    }

    pub fn new_fact_imply (fact: Fact<'a>, imply: std::cell::UnsafeCell<&'a Fact<'a>>) -> Self {
        Fact {
            exprt: fact.exprt,
            value: fact.value,
            imply: Some(imply),
        }
    }

    /// The `new_rev` constructor function returns a true axiom.

    pub fn new_rev (exprt: String) -> Self {
        Fact {
            exprt: exprt,
            value: true,
            imply: None,
        }
    }
}

impl <'a> std::ops::Deref for Fact<'a> {
    type Target = bool;

    /// The `deref` function returns information if the axiom is true.

    fn deref(&self) -> &bool {
        &self.value
    }
}

impl <'a> std::ops::Add for Fact<'a> {
    type Output = Fact<'a>;

    /// The `add` function returns axiom AND axiom.

    #[cfg(not(feature = "and"))]
    fn add(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + &other.exprt + ")",
            value: self.value && other.value,
            imply: None,
        }
    }

    #[cfg(feature = "and")]
    fn add(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + "+" + &other.exprt + ")",
            value: self.value && other.value,
            imply: None,
        }
    }
}

impl <'a> std::ops::BitOr for Fact<'a> {
    type Output = Fact<'a>;

    /// The `bitor` function returns axiom OR axiom.

    fn bitor(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + "|" + &other.exprt + ")",
            value: self.value || other.value,
            imply: None,
        }
    }
}

impl <'a> std::ops::BitXor for Fact<'a> {
    type Output = Fact<'a>;

    /// The `bitxor` function returns axiom XOR axiom.

    fn bitxor(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + "^" + &other.exprt + ")",
            value: self.value ^ other.value,
            imply: None,
        }
    }
}

impl <'a> std::ops::Not for Fact<'a> {
    type Output = Fact<'a>;

    /// The `not` function returns not axiom.

    fn not(self) -> Fact <'a> {
        Fact {
            exprt: "!".to_string() + &self.exprt,
            value: !self.value,
            imply: None,
        }
    }
}

impl <'a> std::fmt::Display for Fact<'a> {

    /// The `fmt` function prints the axiom.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
		write!(f, "{} => {}", self.exprt, self.value)
	}
}

impl <'a> Default for Fact<'a> {

    /// The `default` constructor function returns a false axiom.

    fn default() -> Self {
		Fact::new("_".to_string())
    }
}
