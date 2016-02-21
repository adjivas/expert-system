use ops::Exp;
use solver;
use parser::{Parser};

/// The tree of the axiom A have to match the `expected` arg
fn test_one_tree_expand(s: &str, expected: &str) {
    println!("\nFor : {:?}", s);
    let result = Parser::parse(&s.to_string());
    match result {
        Some(expr) => {
        	let trees = solver::solve(&expr);
        	let res_str = &trees[&'A'].borrow().get_ident().unwrap();
        	println!("Result:   {:?}", res_str);
        	println!("Expected: {:?}", expected);
        	assert!(res_str == expected);
        },
        None => panic!("The expr #{:?}# is false.", s),
    };
}

#[test]
fn test_multi_cmd() {
	let s = "
	B => A
	?A";
	test_one_tree_expand(s, "B=>A");
	let s = "
	B => A
	C => A
	D => A
	?A";
	test_one_tree_expand(s, "((B|C)|D)=>A");
	let s = "
	B => A
	C + D => A
	E => A
	?A";
	test_one_tree_expand(s, "((B|(C+D))|E)=>A");
	let s = "
	B => A
	C + D => A
	E => A
	F => G
	?A";
	test_one_tree_expand(s, "((B|(C+D))|E)=>A");
}

#[test]
fn test_recursive_rule_dependence() {

	let s = "
	B => A
	C => B
	?A";
	test_one_tree_expand(s, "C=>A");

	let s = "
	B => A
	C + D => B
	?A";
	test_one_tree_expand(s, "(C+D)=>A");

	let s = "
	B => A
	C + D => B
	E => A
	?A";
	test_one_tree_expand(s, "((C+D)|E)=>A");

	let s = "
	B => A
	C + D => B
	E => C
	?A";
	test_one_tree_expand(s, "(E+D)=>A");

	let s = "
	B => A
	C + D => B
	E | F => C
	?A";
	test_one_tree_expand(s, "((E|F)+D)=>A");

	let s = "
	B + C => A
	D | E => B
	B => C
	?A";
	test_one_tree_expand(s, "((D|E)+(D|E))=>A");

	let s = "
	B + C => A
	D ^ E => B
	B => C
	?A";
	test_one_tree_expand(s, "((D^E)+(D^E))=>A");

	let s = "
	E | B + C => A
	(F | G) + H => A
	?A";
	test_one_tree_expand(s, "(((E|B)+C)|((F|G)+H))=>A");

	let s = "
	B => A
	C => B
	D => B
	?A";
	test_one_tree_expand(s, "(C|D)=>A");
}
