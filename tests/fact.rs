// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

#[test]
fn test_fact_constructor () {
    assert_eq!(expert_sys::Fact::default(), expert_sys::Fact {
        ident: "_".to_string(),
        value: false,
    });
    assert_eq!(expert_sys::Fact::new("a".to_string()), expert_sys::Fact {
        ident: "a".to_string(),
        value: false,
    });
    assert_eq!(expert_sys::Fact::new_rev("a".to_string()), expert_sys::Fact {
        ident: "a".to_string(),
        value: true,
    });
}

#[test]
fn test_fact_and () {
    assert_eq!(
        {expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new("b".to_string())},
        expert_sys::Fact {
            ident: "ab".to_string(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) +
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            ident: "ab".to_string(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            ident: "ab".to_string(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) +
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            ident: "ab".to_string(),
            value: true,
        }
    );
}

#[test]
fn test_fact_or () {
    assert_eq!(
        {expert_sys::Fact::new("a".to_string()) |
        expert_sys::Fact::new("b".to_string())},
        expert_sys::Fact {
            ident: "a|b".to_string(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) |
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            ident: "a|b".to_string(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) |
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            ident: "a|b".to_string(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) |
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            ident: "a|b".to_string(),
            value: true,
        }
    );
}

#[test]
fn test_fact_xor () {
    assert_eq!(
        {expert_sys::Fact::new("a".to_string()) ^
        expert_sys::Fact::new("b".to_string())},
        expert_sys::Fact {
            ident: "a^b".to_string(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) ^
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            ident: "a^b".to_string(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) ^
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            ident: "a^b".to_string(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) ^
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            ident: "a^b".to_string(),
            value: false,
        }
    );
}
