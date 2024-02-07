use std::fmt;
use std::fmt::{Formatter};


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
		let _percentage = self.value / self.max;
		self.messenger.send("Current usage {percentage}");
	}
}


#[derive(PartialEq, Debug)]
pub struct Clock {
	hours: i32,
	mins: i32,
}

impl fmt::Display for Clock {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{:02}:{:02}", self.hours, self.mins)
	}
}

impl Clock {
	pub fn new(hours: i32, minutes: i32) -> Self {
		let mut c = Clock {hours, mins: 0 };
		c = c.add_minutes(minutes);
		c
	}

	pub fn add_minutes(mut self, minutes: i32) -> Self {
		let hours = self.hours;
		let mut total_mins = hours * 60 + minutes;
		if total_mins < 0 {
			total_mins += (1 - total_mins / (24 * 60)) * 24 * 60
		}
		self.mins = total_mins % 60;
		self.hours = total_mins / 60 % 24;
		self
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
