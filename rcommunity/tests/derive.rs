use rcommunity::{UserType, ID};
use serde::Serialize;

#[derive(Clone, ID, UserType, Serialize)]
struct User(String);
