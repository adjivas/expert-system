use std::rc::Rc;
use std::cell::RefCell;

trait Foo {
    fn foo();
}

struct Bar {
	a: i32
}

impl Foo for Bar {
    fn foo() {
        unimplemented!();
    }
}

fn main() {
	let foos: Vec<Rc<RefCell<Foo>>> = Vec::new();
	let var = Rc::new(RefCell::new(Bar{a: 42}));
	foos.push(var);
}
