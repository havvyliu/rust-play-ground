use std::cell::RefCell;
use std::rc::Rc;

mod dynamic;
mod decimal;
pub mod generics;
pub mod mutex;
pub mod concurrent;
pub mod exercism;


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




	let mut x = Some(2);
	let y = x.take();
	let z = x.as_mut();
	let d = 2;
	assert_eq!(x, None);
	assert_eq!(y, Some(2));
}

#[test]
fn ref_rc_stuff() {
	let a = Rc::new(RefCell::new("a".to_owned()));
	let mut a_cloned = Rc::clone(&a);
	let c = &mut a_cloned;
	unsafe { (*a_cloned.as_ptr()).push('!'); }
	a_cloned.borrow_mut().push('?');
	println!("{:?}", a);
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


#[test]
fn test_iterator() {
	let vec1 = vec![1, 2, 3];
	let vec2 = vec![4, 5, 6];

	let mut iter = vec1.iter();
	println!("Find {:?} in vec1", iter.find(|&x| x == &2));
}

struct Point {
	x: i32,
	y: i32,
}

impl From<[i32; 2]> for Point{
	fn from(value: [i32; 2]) -> Self {
		Point { x: value[0], y: value[1] }
	}
}


#[test]
fn test_from() {
	let arr = [1, 2];
	let pt: Point = Point::from(arr);
}



macro_rules! calculate {
    (eval $e:expr) => {
        {
            // let val: usize = $e; // Force types to be unsigned integers
            println!("{} = {}", stringify!{$e}, $e as usize);
        }
    };
}

#[test]
fn test_macro() {
	calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }
	calculate! {
        eval (1 + 2) * (3 / 4)
    }
}


enum List {
	Cons(i32, Rc<List>),
	Nil,
}

