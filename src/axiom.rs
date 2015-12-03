// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Axiom` structure is a binary axiom.

pub struct Axiom<'a> {
    exprt: char, // logical expression.
    value: bool, // result.
    imply: Option<*mut Axiom<'a>>, // dependencies.
}

impl <'a> Axiom<'a> {

    /// The `new` constructor function returns a default false axiom.

    pub fn new (exprt: char) -> Self {
        Axiom {
            exprt: exprt,
            value: false,
            imply: None,
        }
    }

    /// The `new_imply` constructor function returns a default false axiom with dependencies.

    pub fn new_imply (exprt: char, imply: *mut Axiom<'a>) -> Self {
        Axiom {
            exprt: exprt,
            value: false,
            imply: Some(imply),
        }
    }

    pub fn new_axiom (fact: Axiom<'a>) -> Self {
        Axiom {
            exprt: fact.exprt,
            value: fact.value,
            imply: fact.imply,
        }
    }

    pub fn new_axiom_imply (fact: Axiom<'a>, imply: *mut Axiom<'a>) -> Self {
        Axiom {
            exprt: fact.exprt,
            value: fact.value,
            imply: Some(imply),
        }
    }

    pub fn get_exprt (&self) -> &char {
        &self.exprt
    }
}

impl <'a> std::ops::Deref for Axiom<'a> {
    type Target = bool;

    /// The `deref` function returns the axiom value.

    fn deref(&self) -> &bool {
        match self.imply {
            Some(imply) => unsafe { &**imply },
            None => &self.value,
        }
    }
}

impl <'a> std::ops::DerefMut for Axiom<'a> {

    /// The `deref` function returns a mutable reference to axion value.

    fn deref_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl <'a> PartialEq for Axiom<'a> {

    /// The `eq` function returns a boolean for our axiom equal another axiom.

    fn eq(&self, other: &Axiom<'a>) -> bool {
        self.exprt == other.exprt &&
        self.value == other.value &&
        match (self.imply, other.imply) {
            (Some(imply), Some(other)) if unsafe { *imply == *other } => true,
            (None, None) => true,
            (_, _) => false,
        }
    }
}

/*
impl <'a> std::ops::Add for Axiom<'a> {
    type Output = Axiom<'a>;

    /// The `add` function returns axiom AND axiom.

    #[cfg(not(feature = "and"))]
    fn add(self, other: Axiom) -> Axiom {
        Axiom {
            exprt: "(".to_string() + &self.exprt + &other.exprt + ")",
            value: self.value && other.value,
            imply: None,
        }
    }

    #[cfg(feature = "and")]
    fn add(self, other: Axiom) -> Axiom {
        Axiom {
            exprt: "(".to_string() + &self.exprt + "+" + &other.exprt + ")",
            value: self.value && other.value,
            imply: None,
        }
    }
}

impl <'a> std::ops::BitOr for Axiom<'a> {
    type Output = Axiom<'a>;

    /// The `bitor` function returns axiom OR axiom.

    fn bitor(self, other: Axiom) -> Axiom {
        Axiom {
            exprt: "(".to_string() + &self.exprt + "|" + &other.exprt + ")",
            value: self.value || other.value,
            imply: None,
        }
    }
}

impl <'a> std::ops::BitXor for Axiom<'a> {
    type Output = Axiom<'a>;

    /// The `bitxor` function returns axiom XOR axiom.

    fn bitxor(self, other: Axiom) -> Axiom {
        Axiom {
            exprt: "(".to_string() + &self.exprt + "^" + &other.exprt + ")",
            value: self.value ^ other.value,
            imply: None,
        }
    }
}

impl <'a> std::ops::Not for Axiom<'a> {
    type Output = Axiom<'a>;

    /// The `not` function returns not axiom.

    fn not(self) -> Axiom <'a> {
        Axiom {
            exprt: "!".to_string() + &self.exprt,
            value: !self.value,
            imply: None,
        }
    }
}
*/
impl <'a> std::fmt::Display for Axiom<'a> {

    /// The `fmt` function prints the axiom.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        match self.imply {
            Some(_) => write!(f, "{} => {}, c => ... => {}", self.exprt, self.value, **self),
            None => write!(f, "{} => {}", self.exprt, self.value),
        }
	}
}

impl <'a> std::fmt::Debug for Axiom<'a> {

    /// The `fmt` function prints the axiom.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        match self.imply {
            Some(imply) => write!(f, "{} => {:?}", self.exprt, unsafe { &*imply }),
            None => write!(f, "{} => {}", self.exprt, self.value),
        }
	}
}

impl <'a> Default for Axiom<'a> {

    /// The `default` constructor function returns a false axiom.

    fn default() -> Self {
        Axiom {
            exprt: '_',
            value: false,
            imply: None,
        }
    }
}
