//! Store format types to help enforce consistent data se/desrialization.

/// Key data format that preserves the original order.
#[derive(Debug, Eq, PartialEq)]
pub struct Key(String);

impl Key {
    pub fn raw(k: impl Into<String>) -> Self {
        Key(k.into())
    }
}

impl From<Key> for String {
    fn from(k: Key) -> Self {
        k.0
    }
}

/// Value data format. Currently the same as JSON.
#[derive(Debug, Eq, PartialEq)]
pub struct Value(String);

impl Value {
    pub fn raw(v: impl Into<String>) -> Self {
        Value(v.into())
    }
}

impl From<Value> for String {
    fn from(k: Value) -> Self {
        k.0
    }
}
