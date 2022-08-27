use super::{to_key, Key, Placeholder};

#[derive(serde::Serialize)]
struct User(String);

#[derive(serde::Serialize)]
struct Item(String);

#[derive(serde::Serialize)]
struct Index {
    user: User,
    item: Item,
}

#[derive(serde::Serialize)]
#[serde(rename = "Index")]
struct Query {
    user: User,
    item: (),
}

#[derive(serde::Serialize)]
#[serde(rename = "Index")]
struct QueryItem {
    user: User,
    item: Placeholder<Item>,
}

#[test]
fn test_key_serialization() {
    assert!(to_key("").unwrap() == Key::raw(b"".to_vec()));
    assert!(to_key(&()).unwrap() == Key::raw(b"".to_vec()));
    assert!(to_key(&User("a".to_string())).unwrap() == Key::raw(b"User:a".to_vec()));
    assert!(to_key(&Placeholder::<User>::new()).unwrap() == Key::raw(b"User:".to_vec()));

    assert!(
        to_key(&Index {
            user: User("a".to_string()),
            item: Item("b".to_string())
        })
        .unwrap()
            == Key::raw(b"Index_User:a_Item:b".to_vec())
    );
    assert!(
        to_key(&Query {
            user: User("a".to_string()),
            item: (),
        })
        .unwrap()
            == Key::raw(b"Index_User:a_".to_vec())
    );
    assert!(
        to_key(&QueryItem {
            user: User("a".to_string()),
            item: Placeholder::new()
        })
        .unwrap()
            == Key::raw(b"Index_User:a_Item:".to_vec())
    );
}

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
