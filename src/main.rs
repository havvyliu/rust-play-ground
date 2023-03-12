
// use crate::mutex::mutex;
//use crate::concurrent::concurrent1;

use std::rc::Rc;

pub mod generics;
pub mod mutex;
pub mod concurrent;

use crate::List::{Cons, Nil};

fn main() {
	//concurrent1();
	// mutex();
	//let user = Some(ShirtColor::Red);
	//test();
	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	let b = Cons(3, Rc::clone(&a));
	let c = Cons(4, Rc::clone(&a));

}


fn statementVsExpression() {
	let y = {
		let x = 3;
		x + 1
	};

	println!("y is {}", y);
}

#[test]
fn test() {
	let mut str = "   ";
	let length = str.len();
	println!("length is {}", length);

	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();
	//assert_eq!(v1_iter.next(), Some(&1))
}

enum List {
	Cons(i32, Rc<List>),
	Nil,
}

