use parse::{Parser};
use exp::{Exp};
use axiom::{Axiom};
use ops::{And, Not, Xor, Or, Imply};
use std::rc::Rc;

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

    test_parsability("A ", false);
    test_parsability("A #blabla", false);
    test_parsability("A B => C #blabla", false);
    test_parsability("=> C #blabla", false);
    test_parsability("A + => C #blabla", false);
    test_parsability("A + B + => C #blabla", false);
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
            let result_tree = expr.get(0).unwrap();
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
            let result_tree = expr.get(0).unwrap();
            println!("tree {:?}", result_tree.get_ident());
            assert!(result_tree.get_ident().unwrap() == s.to_string());
        },
        None => panic!("The expr #{:?}# is false.", s),
    };
}

#[test]
fn test_parse_tree() {
    let tree = Imply::new(
        Axiom::new('A') as Rc<Exp>,
        Axiom::new('B') as Rc<Exp>
        );
    test_tree("A => B", tree);

    test_tree2("(A|(B+C))=>D");
    test_tree2("((!(A)+!(B))+!(C))=>D");
    test_tree2("(!((A+C))|D)=>D");
}
