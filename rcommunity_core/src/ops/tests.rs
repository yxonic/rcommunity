use serde::{Deserialize, Serialize};

use crate::ops::Reactor;
use crate::store::memory::MemoryStore;
use crate::store::Store;
use crate::{ItemType, Once, ReactionType, Serializable, UserType, ID};

use super::{ReactionInfo, ReactionInfoOnce};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
struct User(usize);
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
struct Item(usize);
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
struct Vote(i64);
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
struct Comment(String);

impl UserType for User {}
impl ID for User {
    fn id(&self) -> String {
        self.0.to_string()
    }

    fn from(id: &str) -> Self {
        User(id.parse().unwrap())
    }
}

impl ItemType for Item {}
impl ID for Item {
    fn id(&self) -> String {
        self.0.to_string()
    }

    fn from(id: &str) -> Self {
        Item(id.parse().unwrap())
    }
}

impl ReactionType for Vote {}
impl Serializable for Vote {
    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(_data: &str) -> Self {
        todo!()
    }
}
impl Once for Vote {}

impl ReactionType for Comment {}
impl Serializable for Comment {
    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(_data: &str) -> Self {
        todo!()
    }
}

#[tokio::test]
async fn test_reaction_info() {
    let mut store = MemoryStore::default();
    let mut txn = store.begin_txn().await.unwrap();

    let user = User(1000);
    let item = Item(2000);

    Comment("hello".to_string())
        .react(&mut txn, "r1", &user, &item)
        .await
        .unwrap();
    Comment("world".to_string())
        .react(&mut txn, "r2", &user, &item)
        .await
        .unwrap();

    let r = Comment::get_reaction_by_id::<User, Item>(&mut txn, "r1")
        .await
        .unwrap();
    assert!(r.user == user);
    assert!(r.item == item);
    assert!(r.reaction == Comment("hello".to_string()));

    Vote(1).react(&mut txn, "r3", &user, &item).await.unwrap();
    let r = Vote::get_reaction_by_id::<User, Item>(&mut txn, "r3")
        .await
        .unwrap();
    assert!(r.reaction == Vote(1));
    let rid = Vote::get_rid(&mut txn, &user, &item).await.unwrap();
    assert!(&rid == "r3");

    Vote(-1).react(&mut txn, "r4", &user, &item).await.unwrap();
    assert!(Vote::get_reaction_by_id::<User, Item>(&mut txn, "r3")
        .await
        .is_err());
    let rid = Vote::get_rid(&mut txn, &user, &item).await.unwrap();
    assert!(&rid == "r4");
}
