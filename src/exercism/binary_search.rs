

pub fn binary_search(array: &[i32], key: i32) -> Option<usize> {
    let mut l= 0;
    let mut r = array.len();
    while l < r {
        let m = (l + r) / 2;
        if array[m] == key {
            return Some(m);
        } else if array[m] < key {
            l = m + 1;
        } else {
            r = m;
        }
    }
    None
}