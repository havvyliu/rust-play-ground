use std::sync::mpsc;
use std::time::Duration;
use std::thread;


#[warn(unused_must_use)]
pub fn concurrent1() {
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
