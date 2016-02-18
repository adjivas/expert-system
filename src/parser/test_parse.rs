// @gbersac, @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/expert-system
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use ops::{And, Not, Xor, Or, Imply, Set, Exp, Axiom, ExpPtr};
use parser::tokenizer::{Token, Tokenizer, TokenInfo};
use parser::Parser;
use std::collections::VecDeque;
use std::rc::Rc;
use parse_result::ParseResult;
use std::fmt::Display;

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

fn test_tree(s: &str) {
    println!("\nFor : {:?}", s);
    let result = Parser::parse(&s.to_string());
    match result {
        Some(expr) => {
            let result_tree: &ExpPtr = expr.get_instrs().get(0).unwrap();
            println!("tree {:?}", result_tree.borrow().get_ident());
            assert!(result_tree.borrow().get_ident().unwrap() == s.to_string());
        },
        None => panic!("The expr #{:?}# is false.", s),
    };
}

#[test]
fn test_parse_tree() {
    test_tree("(A|(B+C))=>D");
    test_tree("((!(A)+!(B))+!(C))=>D");
    test_tree("(!((A+C))|D)=>D");
}
