use std::collections::HashMap;
use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//
//     let counter = |lines: &[&str]|  {
//         let mut map = HashMap::new();
//         for line in lines {
//             for c in line.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()) {
//                 *map.entry(c).or_insert(0) += 1;
//             }
//         }
//         map
//     };
//
//     match input.len() {
//         0 => HashMap::new(),
//         _ => thread::scope(|s| {
//             let mut handles = Vec::with_capacity(worker_count);
//             for lines in input.chunks(input.len() / worker_count + 1) {
//                 handles.push(s.spawn(|| counter(lines)))
//             }
//
//             let mut map = handles.pop().unwrap().join().unwrap();
//             for res in handles {
//                 res.join().unwrap().into_iter().for_each(|(k, v)| {
//                     *map.entry(k).or_insert(0) += v;
//                 })
//             }
//             map
//         })
//     }
// }

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut map = HashMap::new();
        for line in input {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *map.entry(c).or_default() += 1;
            }
        }
        map
    };

    // redirect to the best implementation.
    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);
            for lines in input.chunks(input.len() / worker_count + 1) {
                handles.push(s.spawn(|| counter(lines)))
            }

            let mut map = handles.pop().unwrap().join().unwrap();
            for res in handles {
                res.join().unwrap().into_iter().for_each(|(k, v)| {
                    *map.entry(k).or_default() += v;
                })
            }

            map
        }),
    }
}



#[test]
fn one_letter() {
    let mut hm = HashMap::new();
    hm.insert('a', 1);
    assert_eq!(crate::exercism::parallel_letter_freq::frequency(&mut ["a"], 4), hm);
}