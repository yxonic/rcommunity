#[derive(Debug, Eq, PartialEq)]
pub struct Key(String);

impl From<Key> for String {
    fn from(k: Key) -> Self {
        k.0
    }
}

impl From<String> for Key {
    fn from(k: String) -> Self {
        Key(k)
    }
}

impl From<&str> for Key {
    fn from(k: &str) -> Self {
        Key(k.to_string())
    }
}

impl From<&String> for Key {
    fn from(k: &String) -> Self {
        Key(k.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Value(String);

impl From<Value> for String {
    fn from(k: Value) -> Self {
        k.0
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value(v.to_string())
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value(v.to_string())
    }
}
