// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use axiom::Axiom;

/// The `Not` structure is a binary Not.

pub struct Not <T>  {
    infer: T, // dependencies.
}

impl <T> Not <T> {

    /// The `new` constructor function returns a default false Not.

    pub fn new (infer: T) -> Self {
        Not {
            infer: infer,
        }
    }
}

impl <'a> Not <*mut Axiom<'a>> {

    /// The `get_value` function returns the value.

    pub fn get_value (&self) -> bool {
        !unsafe { **self.infer }
    }

    /// The `get_exprt` function returns the arithmetic expression.

    fn get_exprt (&self) -> &char {
        unsafe { &*self.infer }.get_exprt()
    }

    /// The `get_infer` function returns the axiom.

    fn get_infer (&self) -> &Axiom<'a> {
        unsafe { &*self.infer }
    }
}

impl <'a> std::fmt::Display for Not<*mut Axiom<'a>> {

    /// The `fmt` function prints the Not.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        write!(f, "!{} => {}", self.get_exprt(), self.get_value())
	}
}

impl <'a> std::fmt::Debug for Not<*mut Axiom<'a>> {

    /// The `fmt` function prints the Not.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        write!(f, "!({:?}) => {}", self.get_infer(), self.get_value())
	}
}

impl <'a> Not<*const Not<*mut Axiom<'a>>> {

    /// The `get_value` function returns the value.

    pub fn get_value (&self) -> bool {
        !unsafe { &*self.infer }.get_value()
    }

    /// The `get_exprt` function returns the arithmetic expression.

    fn get_exprt (&self) -> &char {
        unsafe { &*self.infer }.get_exprt()
    }

    /// The `get_infer` function returns the axiom.

    fn get_infer (&self) -> &Not<*mut Axiom<'a>> {
        unsafe { &*self.infer }
    }
}

impl <'a> std::fmt::Display for Not<*const Not<*mut Axiom<'a>>> {

    /// The `fmt` function prints the Not.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        write!(f, "!{} => {}", self.get_exprt(), self.get_value())
	}
}

impl <'a> std::fmt::Debug for Not<*const Not<*mut Axiom<'a>>> {

    /// The `fmt` function prints the Not.

	fn fmt (
	    &self,
	    f: &mut std::fmt::Formatter,
	) -> Result<(), std::fmt::Error> {
        write!(f, "!({:?}) => {}", *self.get_infer(), self.get_value())
	}
}
