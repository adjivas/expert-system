// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may And be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;

/// The `And` structure is a binary And.

pub struct And <T>  {
    infer: T, // dependencies.
    other: T, // other dependencies.
}

impl <T> And <T> {

    /// The `new` constructor function returns a default false And.

    pub fn new (infer: T, other: T) -> Self {
        And {
            infer: infer,
            other: other,
        }
    }
}

impl <'a> And <*mut Axiom<'a>> {

    /// The `get_value` function returns the value.

    pub fn get_value (&self) -> bool {
        *unsafe { &**self.infer } &&
        *unsafe { &**self.other }
    }

    /// The `get_exprt` function returns the two arithmetic expression.

    fn get_exprt (&self) -> (&char, &char) {
        (
            unsafe { &*self.infer }.get_exprt(),
            unsafe { &*self.other }.get_exprt()
        )
    }

    /// The `get_infer` function returns the two axiom.

    fn get_infer (&self) -> (&Axiom<'a>, &Axiom<'a>) {
        (
            unsafe { &*self.infer },
            unsafe { &*self.other }
        )
    }
}

impl <'a> std::fmt::Display for And<*mut Axiom<'a>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other): (&char, &char) = self.get_exprt();

        write!(f, "({}+{}) => {}", infer, other, self.get_value())
	}
}

impl <'a> std::fmt::Debug for And<*mut Axiom<'a>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other) = self.get_infer();

        write!(f, "(({:?})+({:?})) => {:?}", infer, other, self.get_value())
	}
}

impl <'a> And<*const And<*mut Axiom<'a>>> {

    /// The `get_value` function returns the value.

    pub fn get_value (&self) -> bool {
        unsafe { &*self.infer }.get_value() &&
        unsafe { &*self.other }.get_value()
    }

    /// The `get_infer` function returns the two axiom.

    fn get_infer (&self) -> (&And<*mut Axiom<'a>>, &And<*mut Axiom<'a>>) {
        (
            unsafe { &*self.infer },
            unsafe { &*self.other }
        )
    }
}

impl <'a> std::fmt::Display for And<*const And<*mut Axiom<'a>>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other): (&And<*mut Axiom<'a>>, &And<*mut Axiom<'a>>) = self.get_infer();

        write!(f, "(({})+({})) => {}", infer, other, self.get_value())
	}
}

impl <'a> std::fmt::Debug for And<*const And<*mut Axiom<'a>>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other) = self.get_infer();

        write!(f, "(({:?})+({:?})) => {:?}", infer, other, self.get_value())
	}
}
