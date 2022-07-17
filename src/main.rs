use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

fn concurrent() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let val = String::from("hi");
		tx.send(val);
		thread::sleep(Duration::from_secs(1));
		let val = String::from("hi1");
		tx.send(val).unwrap();
		let val = String::from("hi2");
		tx.send(val).unwrap();
	});

	for received in rx {
		println!("Got : {}", received);
	}
}
fn main() {
	// concurrent();
	mutex();
}

fn statementVsExpression() {
	let y = {
		let x = 3;
		x + 1
	};

	println!("y is {}", y);
}

fn test() {
	let mut str = "   ";
	let length = str.len();
	println!("length is {}", length);
	*str = "a";
}


fn mutex() {
	let m = Mutex::new(5);

	{
		let mut num = m.lock().unwrap();
		*num = 6;
	}

	println!("m = {:?}", m)
}
