use parse::{Parser};
use exp::{Exp};
use ops::{And, Not, Xor, Or, Imply};

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

    test_parsability("(A + B => C", false);
}

// fn test_tree(s: &str, tree: Box<Exp>) {
// 	println!("\nFor : {:?}", s);
// 	let result = Parser::parse(&s.to_string());
// 	match result {
// 	    Some(expr) => {
// 	    	let result = expr[0];
// 	    	assert!(*result == tree);
// 	    },
// 	    None => panic!("The expr #{:?}# is false.", s),
// 	};
// }
