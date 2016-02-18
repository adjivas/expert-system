use ops::{And, Not, Xor, Or, Imply, Set, Exp, Axiom, ExpPtr};
use parser::Parser;

fn test_one(s: &str, expected_true_axiom: &str) {
    println!("\nFor : {:?}", s);
    let result = Parser::parse(&s.to_string());
    match result {
        Some(expr) => {
            let instruction: &ExpPtr = expr.get_instrs().get(0).unwrap();
            let mut result_values = Set::new();
            let expected_values = Set::from_str(expected_true_axiom);
            let initial_value = &expr.get_initial_value()[0];
            instruction.borrow().solve(initial_value, &mut result_values);
            print!("result {}", result_values);
            assert!(result_values == expected_values);
        },
        None => panic!("The expr #{:?}# is false.", s),
    };
}

#[test]
fn test_expression_solve() {
    let s = "
    A => B
    =A";
    test_one(s, "B");
    let s = "
    A + B => C
    =A";
    test_one(s, "");
    let s = "
    A + B => C
    =AB";
    test_one(s, "C");
    let s = "
    (A + B) | C => D + E
    =AB";
    test_one(s, "DE");
    let s = "
    (A + B) | C => D + E
    =C";
    let s = "
    A => !B
    =";
    test_one(s, "B");
    let s = "
    A ^ B => C
    =A";
    test_one(s, "C");
    let s = "
    A ^ B => C
    =AB";
    test_one(s, "");
}

#[test]
#[should_panic]
fn test() {
    let s = "
    A + B => C
    =A";
    test_one(s, "C");
    ;
}
