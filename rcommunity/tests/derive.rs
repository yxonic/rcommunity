use rcommunity::{UserType, ID};

#[derive(Clone, ID, UserType)]
struct User(String);
