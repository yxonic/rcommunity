use rcommunity::{Unique, UserType};

#[derive(Clone, Unique, UserType)]
struct User(String);

#[test]
fn test() {
    assert!(true);
}
