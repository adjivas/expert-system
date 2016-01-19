use parse::{Parser};

fn test_one(s: &str, is_correct: bool) {
	let result = Parser::parse(&s.to_string());
	println!("For : {:?}", s);
	if is_correct {
	    assert!(result.is_some());
	} else {
	    assert!(result.is_none());
	}
}

#[test]
fn test_parser_basics() {
    test_one("A => B", true);
    test_one("A => B #blabla", true);
    test_one("", true);
    test_one("A ", false);
    test_one("A #blabla", false);
    test_one("A B => C #blabla", false);
}
