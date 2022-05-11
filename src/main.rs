use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let stack_id: i8 = 10;
    let null: i8;
    let string: &str = "asdasd";
    let heap_vector: Vec<i8> = vec![];
    let heap_string: String = String::from("Hello");

    let stack_id_ref = stack_id;
    println!("{}", stack_id);
    println!("{}", stack_id_ref);

    // Ownership changed!!!!!!!!!!!
    // let heap_string_ref = heap_string;
    // Clone...
    let heap_string_clone = heap_string.clone();
    // Borrowing...
    let heap_string_borrow = &heap_string;
    println!("{}", heap_string);

	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("number {} from spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});

	handle.join().unwrap();

	for i in 1..5 {
		println!("number {} from main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}

	
}
