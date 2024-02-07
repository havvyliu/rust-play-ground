use std::rc::Rc;

mod dynamic;
pub mod generics;
pub mod mutex;
pub mod concurrent;
mod exercism;

use crate::List::{Cons, Nil};

struct WrappedVal<T> {
    value: T,
}

impl<T> WrappedVal<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let y = WrappedVal { value: "str"};
    println!("{}", y.get_value());
	// concurrent1();
	// mutex();
	//let user = Some(ShirtColor::Red);
	//test();
	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	let _b = Cons(3, Rc::clone(&a));
	let _c = Cons(4, Rc::clone(&a));
	let _d: &str;
}

#[test]
fn test() {
	let str = "   ";
	let length = str.len();
	println!("length is {}", length);

	let v1 = vec![1, 2, 3];
	let _v1_iter = v1.iter();
	//assert_eq!(v1_iter.next(), Some(&1))
}

enum List {
	Cons(i32, Rc<List>),
	Nil,
}

