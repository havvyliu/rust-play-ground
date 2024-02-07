use std::sync::mpsc;
use std::thread;


pub fn concurrent1() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		for i in 1..1000 {
			let val = String::from("hi");
			tx.send(val).unwrap();
			let val = String::from("hi1");
			tx.send(val).unwrap();
			let val = String::from("hi2");
			tx.send(val).unwrap();
		}
	});

	for received in rx {
		println!("Got : {}", received);
	}
}

#[test]
pub fn test_concurrency() {
	concurrent1();
}
