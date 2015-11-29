// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Fact` structure is a binary axiom.

#[derive(Debug, PartialEq)]
pub struct Fact<'a> {
    pub exprt: String, // logical expression.
    pub imply: Vec<&'a Fact<'a>>, // dependencies.
    pub value: bool, // result.
}

impl <'a> Fact<'a> {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (exprt: String) -> Self {
        Fact {
            exprt: exprt,
            imply: Vec::new(),
            value: false,
        }
    }

    /// The `new_rev` constructor function returns a true axiom.

    pub fn new_rev (exprt: String) -> Self {
        Fact {
            exprt: exprt,
            imply: Vec::new(),
            value: true,
        }
    }

    /// The `push_imply` adds a new dependency to the tree.

    pub fn push_imply (&mut self, imply: &'a Fact) {
        self.imply.push(imply);
    }
}


impl <'a> std::ops::Deref for Fact<'a> {
    type Target = bool;

    /// The `deref` function returns information if the axiom is true.

    #[cfg(not(feature = "verbose"))]
    fn deref(&self) -> &bool {
        if self.value { &self.value }
        else {
            match self.imply.iter().find(|&val| ***val) {
                Some(val) => &val,
                None => &self.value,
            }
        }
    }

    /// The `deref` function returns information if the axiom is true.

    #[cfg(feature = "verbose")]
    fn deref(&self) -> &bool {
        if self.value {
            println!("{}", super::FACT_DEREF_LAST.to_string() + &self.exprt + &" is true. ");
            &self.value
        }
        else {
            match self.imply.iter().find(|&val| ***val) {
                Some(val) => {
                    println!("{}", super::FACT_FIRSTS_LAST.to_string() + &self.exprt + &", ");
                    &val
                },
                None => &self.value,
            }
        }
    }
}

impl <'a> std::ops::Add for Fact<'a> {
    type Output = Fact<'a>;

    /// The `add` function returns axiom AND axiom.

    #[cfg(not(feature = "and"))]
    fn add(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + &other.exprt + ")",
            imply: Vec::new(),
            value: self.value && other.value,
        }
    }

    #[cfg(feature = "and")]
    fn add(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + "&" + &other.exprt + ")",
            imply: Vec::new(),
            value: self.value && other.value,
        }
    }
}

impl <'a> std::ops::BitOr for Fact<'a> {
    type Output = Fact<'a>;

    /// The `bitor` function returns axiom OR axiom.

    fn bitor(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + "|" + &other.exprt + ")",
            imply: Vec::new(),
            value: self.value || other.value,
        }
    }
}

impl <'a> std::ops::BitXor for Fact<'a> {
    type Output = Fact<'a>;

    /// The `bitxor` function returns axiom XOR axiom.

    fn bitxor(self, other: Fact) -> Fact {
        Fact {
            exprt: "(".to_string() + &self.exprt + "^" + &other.exprt + ")",
            imply: Vec::new(),
            value: self.value ^ other.value,
        }
    }
}

impl <'a> std::ops::Not for Fact<'a> {
    type Output = Fact<'a>;

    /// The `not` function returns not axiom.

    fn not(self) -> Fact <'a> {
        Fact {
            exprt: "!".to_string() + &self.exprt,
            imply: Vec::new(),
            value: !self.value,
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
