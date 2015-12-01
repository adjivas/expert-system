// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

fn main () {
    let a: expert_sys::Fact = expert_sys::Fact::new_rev("a".to_string());
    let b: expert_sys::Fact = expert_sys::Fact::new_imply("b".to_string(), std::cell::UnsafeCell::new(&a));
    let c: expert_sys::Fact = expert_sys::Fact::new_imply("c".to_string(), std::cell::UnsafeCell::new(&a));
}
