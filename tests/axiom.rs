// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

#[test]
fn test_example () {
    let mut axioms: expert_sys::Set = expert_sys::Set::default();

    axioms.set_value('a', true);
    axioms.set_imply('b', 'a');
    assert_eq!(axioms.get_value('a'), Some(true));
    assert_eq!(axioms.get_ident('a'), Some("a".to_string()));
    assert_eq!(axioms.get_value('b'), Some(true));
    assert_eq!(axioms.get_ident('b'), Some("b=>a".to_string()));
    assert_eq!(axioms.get_value('c'), Some(false));
    assert_eq!(axioms.get_ident('c'), Some("c".to_string()));
    assert_eq!(axioms.get_value('_'), None);
}
