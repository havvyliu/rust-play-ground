use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Deref;

pub fn anagrams_for<'a>(word: & str, possible_anagrams: &'a[& str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    for x in possible_anagrams {
        if same(word, x) {
            set.insert(x.deref());
        }
    }
    set
}

fn same(word: &str, s: &str) -> bool {
    let mut w: Vec<char> = word.chars().flat_map(|c| c.to_lowercase()).collect();
    let mut s_lower: Vec<char> = s.chars().flat_map(|c| c.to_lowercase()).collect();
    if w == s_lower {
        return false;
    } else {
        w.sort();
        s_lower.sort();
        return w == s_lower;
    }
}

#[test]
#[ignore]
fn anagrams_must_use_all_letters_exactly_once() {
    let word = "listen";
    let inputs = &["enlists", "google", "inlets", "banana"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["inlets"]);
    assert_eq!(output, expected);
}


#[test]
#[ignore]
fn same_bytes() {
    let word = "a⬂";
    let inputs = &["€a"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}
