trait Tr: PartialEq {
	fn f1();
}

#[derive(PartialEq)]
struct St ;

impl Tr for St {
	fn f1() {
	    unimplemented!()
	}
}

fn main() {
	let var : std::rc::Rc<Tr> = St;
}
