use rcommunity::UserType;

#[derive(UserType)]
struct User(String);

#[test]
fn test() {
    assert!(true);
}
