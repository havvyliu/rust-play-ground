use std::collections::HashMap;
#[macro_export]
macro_rules! hashmap {
    ( $( $key: expr => $val:expr ), * $(,)?) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

#[test]
fn test_hash_map() {
    let map = hashmap!('a' => 5, 'b' => 7, );
    let empty_map: ::std::collections::HashMap<(), ()> = hashmap!();
    let _with_trailing = hashmap!(23 => 623, 34 => 21,);
    let _without_comma = hashmap!(23=> 623, 34 => 21);
    println!("{:?}", map);
    println!("{:?}", empty_map);
}

#[test]
fn empty() {
    let expected: HashMap<u32, u32> = HashMap::new();
    let computed: HashMap<u32, u32> = hashmap!();
    assert_eq!(computed, expected);
}

#[test]
#[ignore]
fn trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert('h', 89);
    expected.insert('a', 1);
    expected.insert('s', 19);
    expected.insert('h', 8);
    assert_eq!(
        hashmap!(
            'h' => 89,
            'a' => 1,
            's' => 19,
            'h' => 8,
        ),
        expected
    );

}