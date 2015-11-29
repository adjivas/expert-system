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
        exprt: "_".to_string(),
        imply: Vec::new(),
        value: false,
    });
    assert_eq!(expert_sys::Fact::new("a".to_string()), expert_sys::Fact {
        exprt: "a".to_string(),
        imply: Vec::new(),
        value: false,
    });
    assert_eq!(expert_sys::Fact::new_rev("a".to_string()), expert_sys::Fact {
        exprt: "a".to_string(),
        imply: Vec::new(),
        value: true,
    });
}

#[test]
fn test_fact_deref () {
    assert_eq!(*expert_sys::Fact::default(), false);
    assert_eq!(*expert_sys::Fact::new("a".to_string()), false);
    assert_eq!(*expert_sys::Fact::new_rev("a".to_string()), true);
}

#[test]
#[cfg(not(feature = "and"))]
fn test_fact_not () {
    assert_eq!(
        !expert_sys::Fact::new("a".to_string()),
        expert_sys::Fact {
            exprt: "!a".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        !(expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new("b".to_string())),
        expert_sys::Fact {
            exprt: "!(ab)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        !(expert_sys::Fact::new("a".to_string()) |
        expert_sys::Fact::new("b".to_string())),
        expert_sys::Fact {
            exprt: "!(a|b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        !(expert_sys::Fact::new("a".to_string()) ^
        expert_sys::Fact::new("b".to_string())),
        expert_sys::Fact {
            exprt: "!(a^b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
}

#[test]
#[cfg(feature = "and")]
fn test_fact_not () {
    assert_eq!(
        !expert_sys::Fact::new("a".to_string()),
        expert_sys::Fact {
            exprt: "!a".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        !(expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new("b".to_string())),
        expert_sys::Fact {
            exprt: "!(a&b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        !(expert_sys::Fact::new("a".to_string()) |
        expert_sys::Fact::new("b".to_string())),
        expert_sys::Fact {
            exprt: "!(a|b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        !(expert_sys::Fact::new("a".to_string()) ^
        expert_sys::Fact::new("b".to_string())),
        expert_sys::Fact {
            exprt: "!(a^b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
}

#[test]
#[cfg(not(feature = "and"))]
fn test_fact_and () {
    assert_eq!(
        {expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new("b".to_string())},
        expert_sys::Fact {
            exprt: "(ab)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) +
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            exprt: "(ab)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(ab)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) +
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(ab)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
}

#[test]
#[cfg(feature = "and")]
fn test_fact_and () {
    assert_eq!(
        {expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new("b".to_string())},
        expert_sys::Fact {
            exprt: "(a&b)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) +
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            exprt: "(a&b)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) +
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(a&b)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) +
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(a&b)".to_string(),
            imply: Vec::new(),
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
            exprt: "(a|b)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) |
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            exprt: "(a|b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) |
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(a|b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) |
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(a|b)".to_string(),
            imply: Vec::new(),
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
            exprt: "(a^b)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) ^
        expert_sys::Fact::new("b".to_string()),
        expert_sys::Fact {
            exprt: "(a^b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new("a".to_string()) ^
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(a^b)".to_string(),
            imply: Vec::new(),
            value: true,
        }
    );
    assert_eq!(
        expert_sys::Fact::new_rev("a".to_string()) ^
        expert_sys::Fact::new_rev("b".to_string()),
        expert_sys::Fact {
            exprt: "(a^b)".to_string(),
            imply: Vec::new(),
            value: false,
        }
    );
}

#[test]
fn test_fact_imply_simple () {
    {
        // b => a
        {
            let b: expert_sys::Fact = expert_sys::Fact::new("b".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            a.push_imply(&b);
            assert_eq!(*b, false);
            assert_eq!(*a, false);
        };
        {
            let b: expert_sys::Fact = expert_sys::Fact::new_rev("b".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            a.push_imply(&b);
            assert_eq!(*b, true);
            assert_eq!(*a, true);
        };
    };
    {
        // b => a
        // c => a
        {
            let b: expert_sys::Fact = expert_sys::Fact::new("b".to_string());
            let c: expert_sys::Fact = expert_sys::Fact::new("c".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            a.push_imply(&b);
            a.push_imply(&c);
            assert_eq!(*b, false);
            assert_eq!(*c, false);
            assert_eq!(*a, false);
        };
        {
            let b: expert_sys::Fact = expert_sys::Fact::new_rev("b".to_string());
            let c: expert_sys::Fact = expert_sys::Fact::new("c".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            a.push_imply(&b);
            a.push_imply(&c);
            assert_eq!(*b, true);
            assert_eq!(*c, false);
            assert_eq!(*a, true);
        };
        {
            let b: expert_sys::Fact = expert_sys::Fact::new("b".to_string());
            let c: expert_sys::Fact = expert_sys::Fact::new_rev("c".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            a.push_imply(&b);
            a.push_imply(&c);
            assert_eq!(*b, false);
            assert_eq!(*c, true);
            assert_eq!(*a, true);
        };
        {
            let b: expert_sys::Fact = expert_sys::Fact::new_rev("b".to_string());
            let c: expert_sys::Fact = expert_sys::Fact::new_rev("c".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            a.push_imply(&b);
            a.push_imply(&c);
            assert_eq!(*b, true);
            assert_eq!(*c, true);
            assert_eq!(*a, true);
        };
    };
    {
        // c => b => a
        {
            let c: expert_sys::Fact = expert_sys::Fact::new("c".to_string());
            let mut b: expert_sys::Fact = expert_sys::Fact::new("b".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            b.push_imply(&c);
            a.push_imply(&b);
            assert_eq!(*b, false);
            assert_eq!(*c, false);
            assert_eq!(*a, false);
        };
        {
            let c: expert_sys::Fact = expert_sys::Fact::new("c".to_string());
            let mut b: expert_sys::Fact = expert_sys::Fact::new_rev("b".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            b.push_imply(&c);
            a.push_imply(&b);
            assert_eq!(*b, true);
            assert_eq!(*c, false);
            assert_eq!(*a, true);
        };
        {
            let c: expert_sys::Fact = expert_sys::Fact::new_rev("c".to_string());
            let mut b: expert_sys::Fact = expert_sys::Fact::new("b".to_string());
            let mut a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());

            b.push_imply(&c);
            a.push_imply(&b);
            assert_eq!(*b, true);
            assert_eq!(*c, true);
            assert_eq!(*a, true);
        };
    };
}

#[test]
fn test_fact_imply_extend () {
    {
        let c: expert_sys::Fact = expert_sys::Fact::new("c".to_string());
        let b: expert_sys::Fact = expert_sys::Fact::new("b".to_string());
        let a: expert_sys::Fact = expert_sys::Fact::new("a".to_string());
        assert_eq!(*c, false);
        assert_eq!(*b, false);
        assert_eq!(*a, false);
        let a_or_b_and_c: expert_sys::Fact = a | b + c;
        let mut e: expert_sys::Fact = expert_sys::Fact::new("e".to_string());

        e.push_imply(&a_or_b_and_c);
        assert_eq!(*a_or_b_and_c, false);
        assert_eq!(*e, false);
    };
}
