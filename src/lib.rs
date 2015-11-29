// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

mod fact;

pub use fact::Fact;
#[cfg(feature = "verbose")] const FACT_DEREF_LAST: &'static str = "We know that ";
#[cfg(feature = "verbose")] const FACT_FIRSTS_LAST: &'static str = "Since we know";
