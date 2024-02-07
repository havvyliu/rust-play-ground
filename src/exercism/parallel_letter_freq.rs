use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut total = HashMap::new();
    let (tx, rx) = mpsc::channel();
    let input_len = input.len();
    let split_size = input_len / worker_count;
    let mut threads = vec![];

    for i in 0..worker_count {
        let tx_cloned = tx.clone();
        let l = i * split_size;
        let r = if i == worker_count - 1 { input_len } else { l + split_size };
        let sliced: Vec<String> = input[l..r].iter().map(|&s| s.to_owned()).collect();

        threads.push(thread::spawn(move || {
            let mut map = HashMap::new();
            for s in sliced {
                for c in s.chars() {
                    if c.is_alphabetic() {*map.entry(c.to_ascii_lowercase()).or_insert(0) += 1}
                }
            }
            tx_cloned.send(map).unwrap();
        }));
    }

    // Explicitly drop the original sender to ensure the channel closes once all senders are out of scope.
    drop(tx);

    // Collect results for exactly `worker_count` messages.
    for _ in 0..worker_count {
        let map = rx.recv().unwrap();
        for (k, v) in map.iter() {
            *total.entry(*k).or_insert(0) += *v;
        }
    }

    // Wait for all threads to complete execution. But it is not needed.
    for thread in threads {
        thread.join().unwrap();
    }

    total
}


#[test]
fn one_letter() {
    let mut hm = HashMap::new();
    hm.insert('a', 1);
    assert_eq!(frequency(&mut ["a"], 4), hm);
}

