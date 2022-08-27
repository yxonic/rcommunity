use super::to_key;

#[test]
fn test_key_ordering() {
    // strings
    assert!(to_key("").unwrap() < to_key(" ").unwrap());
    assert!(to_key("a").unwrap() < to_key("b").unwrap());
    assert!(to_key("user:a").unwrap() < to_key("user:b").unwrap());

    // ints
    assert!(to_key(&-3).unwrap() < to_key(&-1).unwrap());
    assert!(to_key(&-1).unwrap() < to_key(&0).unwrap());
    assert!(to_key(&-1).unwrap() < to_key(&1u8).unwrap());
    assert!(to_key(&1u32).unwrap() < to_key(&2u8).unwrap());
    assert!(to_key(&2u32).unwrap() == to_key(&2i32).unwrap());

    // floats
    assert!(to_key(&-1e12).unwrap() < to_key(&-1e-12).unwrap());
    assert!(to_key(&-3.).unwrap() < to_key(&0.).unwrap());
    assert!(to_key(&0.3).unwrap() < to_key(&0.4).unwrap());
    assert!(to_key(&0.04).unwrap() < to_key(&0.4).unwrap());
    assert!(to_key(&9e22).unwrap() < to_key(&1e23).unwrap());
}
