// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may And be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;

/// The `And` structure is a binary And.

pub struct And <A, B>  {
    infer: A, // dependencies.
    other: B, // other dependencies.
}

impl <A, B> And <A, B> {

    /// The `new` constructor function returns a default false And.

    pub fn new (infer: A, other: B) -> Self {
        And {
            infer: infer,
            other: other,
        }
    }
}

impl <'a, 'b> And <*mut Axiom<'a>, *mut Axiom<'b>> {

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

    fn get_infer (&self) -> (&Axiom<'a>, &Axiom<'b>) {
        (
            unsafe { &*self.infer },
            unsafe { &*self.other }
        )
    }
}

impl <'a, 'b> std::fmt::Display for And<*mut Axiom<'a>, *mut Axiom<'b>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other) = self.get_exprt();

        write!(f, "({}+{}) => {}", infer, other, self.get_value())
	}
}

impl <'a, 'b> std::fmt::Debug for And<*mut Axiom<'a>, *mut Axiom<'b>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other) = self.get_infer();

        write!(f, "(({:?})+({:?})) => {:?}", infer, other, self.get_value())
	}
}

impl <'a1, 'a2, 'b1, 'b2> And<*const And<*mut Axiom<'a1>, *mut Axiom<'a2>>, *const And<*mut Axiom<'b1>, *mut Axiom<'b2>>> {

    /// The `get_value` function returns the value.

    pub fn get_value (&self) -> bool {
        unsafe { &*self.infer }.get_value() &&
        unsafe { &*self.other }.get_value()
    }

    /// The `get_infer` function returns the two axiom.

    fn get_infer (&self) -> (&And<*mut Axiom<'a1>, *mut Axiom<'a2>>, &And<*mut Axiom<'b1>, *mut Axiom<'b2>>) {
        (
            unsafe { &*self.infer },
            unsafe { &*self.other }
        )
    }
}

impl <'a1, 'a2, 'b1, 'b2> std::fmt::Display for And<*const And<*mut Axiom<'a1>, *mut Axiom<'a2>>, *const And<*mut Axiom<'b1>, *mut Axiom<'b2>>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other) = self.get_infer();

        write!(f, "(({})+({})) => {}", infer, other, self.get_value())
	}
}

impl <'a1, 'a2, 'b1, 'b2> std::fmt::Debug for And<*const And<*mut Axiom<'a1>, *mut Axiom<'a2>>, *const And<*mut Axiom<'b1>, *mut Axiom<'b2>>> {

    /// The `fmt` function prints the And.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        let (infer, other) = self.get_infer();

        write!(f, "(({:?})+({:?})) => {:?}", infer, other, self.get_value())
	}
}
