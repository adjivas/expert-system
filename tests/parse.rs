// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate expert_sys;

use expert_sys::ops::{And, Not, Xor, Or, Imply};
use expert_sys::Token;
use expert_sys::TokenInfo;
use expert_sys::Tokenizer;
use expert_sys::Exp;
use expert_sys::Axiom;
use expert_sys::Parser;
use std::collections::VecDeque;
use std::rc::Rc;
use expert_sys::Set;
use expert_sys::Rules;

fn test_parsability(s: &str, is_correct: bool) {
	println!("\nFor : {:?}", s);
	let result = Parser::parse(&s.to_string());
	if is_correct {
	    assert!(result.is_some());
	} else {
	    assert!(result.is_none());
	}
}

#[test]
fn test_parser_basics() {
    test_parsability("", true);
    test_parsability("A => B", true);
    test_parsability("A + B => C", true);
    test_parsability("A + B + C => D", true);
    test_parsability("A + B + C + D => E", true);
    test_parsability("A + B + !C + !D => E", true);
    test_parsability("A | B => C", true);
    test_parsability("A | B + !C | !D => E", true);
    test_parsability("A ^ B => C", true);
    test_parsability("A ^ B ^ !C + !D => E", true);
    test_parsability("!A => !B", true);
    test_parsability("!(A + B) => C", true);
    test_parsability("A => B #blabla", true);
    // test_parsability("?ABCDEF #blabla", true);
    // test_parsability("=ABCDEF #blabla", true);

    test_parsability("A ", false);
    test_parsability("A #blabla", false);
    test_parsability("A B => C #blabla", false);
    test_parsability("=> C #blabla", false);
    test_parsability("A + => C #blabla", false);
    test_parsability("A + B + => C #blabla", false);
    test_parsability("=ABCD|EF #blabla", false);
}

#[test]
fn test_parser_parenthesis() {
    test_parsability("(A) => C", true);
    test_parsability("((A)) => C", true);
    test_parsability("(A + B) => C", true);
    test_parsability("(A + B) ^ (C + D) => E", true);
    test_parsability("A | (B + C) => D", true);

    test_parsability("(A + B => C", false);
}

fn test_tree(s: &str, tree: Rc<Imply>) {
    println!("\nFor : {:?}", s);
    let result = Parser::parse(&s.to_string());
    match result {
        Some(expr) => {
            let result_tree = expr.get_instrs().get(0).unwrap();
            println!("tree {:?}", result_tree.get_ident());
            assert!(result_tree.eq(tree as Rc<Exp>));
        },
        None => panic!("The expr #{:?}# is false.", s),
    };
}

fn test_tree2(s: &str) {
    println!("\nFor : {:?}", s);
    let result = Parser::parse(&s.to_string());
    match result {
        Some(expr) => {
            let result_tree = expr.get_instrs().get(0).unwrap();
            println!("tree {:?}", result_tree.get_ident());
            assert!(result_tree.get_ident().unwrap() == s.to_string());
        },
        None => panic!("The expr #{:?}# is false.", s),
    };
}

#[test]
fn test_parse_tree() {
    let tree = Imply::new (
        Axiom::new('A') as Rc<Exp>,
        Axiom::new('B') as Rc<Exp>
    );
    test_tree("A => B", tree);

    test_tree2("(A|(B+C))=>D");
    test_tree2("((!(A)+!(B))+!(C))=>D");
    test_tree2("(!((A+C))|D)=>D");
}
