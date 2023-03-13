pub trait Messenger {
	fn send(&self, msg: &str);
}

pub struct LimitTracker<T: Messenger> {
	messenger: T,
	value: usize,
	max: usize,
}

impl<T> LimitTracker<T> 
where 
	T: Messenger, {

	pub fn new(messenger: T, max: usize) -> LimitTracker<T>{
		LimitTracker{
			messenger,
			value: 0,
			max,
		}
	}

	pub fn set_value(&mut self, value: usize) {
		self.value = value;
		let percentage = self.value / self.max;
		self.messenger.send("Current usage {percentage}");
	}


	fn test<'a>() -> &'a str {
		let a = "test";
		&a
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	

	#[test]
	fn messing_with_rc() {
		let a = Rc::new(1);
		let b = Rc::clone(&a);
		assert_eq!(2, Rc::strong_count(&a));
	}

}
