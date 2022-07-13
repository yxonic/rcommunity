//! Type-safe API for accessing community store and operations.

use std::marker::PhantomData;

use crate::{
    error::Result,
    markers::{ItemType, ReactionType, UserType},
    ops::Reactor,
    store::{Store, Transaction},
};

#[derive(Debug)]
pub struct UserItemUnboundedReactionClient<
    'store,
    TS: Store,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    store: &'store mut TS,
    user: TU,
    item: TI,
    reaction_type: PhantomData<TR>,
}

impl<'store, TS: Store, TU: UserType, TI: ItemType, TR: ReactionType>
    UserItemUnboundedReactionClient<'store, TS, TU, TI, TR>
{
    /// Create a new reaction.
    ///
    /// # Errors
    /// Will return error when internal store failed.
    pub async fn react(&mut self, reaction: impl Into<TR>) -> Result<String> {
        let r: TR = reaction.into();
        let mut txn = self.store.begin_txn().await?;
        let rid = uuid::Uuid::new_v4().to_string(); // TODO: keep Uuid type
        r.react(&mut txn, &rid, &self.user, &self.item).await?;
        txn.commit().await?;
        Ok(rid)
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::{
        markers::{ItemType, Once, ReactionType, Serializable, UserType, ID},
        store::{memory::MemoryStore, Key, Store, Transaction},
    };

    use super::UserItemUnboundedReactionClient;

    // test types
    #[derive(Clone, Debug)]
    struct User(String);
    impl ID for User {
        fn id(&self) -> &str {
            &self.0
        }
        fn from(id: &str) -> Self {
            User(id.into())
        }
    }
    impl UserType for User {}
    #[derive(Clone, Debug)]
    struct Item(String);
    impl ID for Item {
        fn id(&self) -> &str {
            &self.0
        }
        fn from(id: &str) -> Self {
            Item(id.into())
        }
    }
    impl ItemType for Item {}

    #[derive(Copy, Clone, Debug)]
    enum Vote {
        Upvote,
        Downvote,
    }
    impl Serializable for Vote {
        fn serialize(&self) -> String {
            match *self {
                Vote::Upvote => "Upvote",
                Vote::Downvote => "Downvote",
            }
            .into()
        }
        fn deserialize(data: &str) -> Self {
            if data.starts_with('U') {
                Vote::Upvote
            } else {
                Vote::Downvote
            }
        }
    }
    impl ReactionType for Vote {}
    impl Once for Vote {}

    #[derive(Clone, Debug)]
    struct Comment(String);
    impl ReactionType for Comment {}
    impl ItemType for Comment {}
    impl ID for Comment {
        fn id(&self) -> &str {
            &self.0
        }
        fn from(id: &str) -> Self {
            Comment(id.into())
        }
    }

    #[tokio::test]
    async fn test_reaction() {
        let mut store = MemoryStore::default();
        let txn = store.begin_txn().await.unwrap();

        let mut client = UserItemUnboundedReactionClient {
            store: &mut store,
            user: User("1000".into()),
            item: Item("2000".into()),
            reaction_type: PhantomData::<Vote>,
        };
        let rid = client.react(Vote::Upvote).await.unwrap();
        // vote tests
        let value: String = txn
            .get(Key::raw("ui_Vote_User:1000_Item:2000"))
            .await
            .unwrap()
            .unwrap()
            .into();
        assert_eq!(value, format!("Upvote_{}", rid));

        let value = txn
            .get(Key::raw("ui_Vote_User:1000_Item:2000_Upvote"))
            .await
            .unwrap();
        assert!(value.is_none());

        client.react(Vote::Downvote).await.unwrap();
        let value: String = txn
            .get(Key::raw("ui_Vote_User:1000_Item:2000"))
            .await
            .unwrap()
            .unwrap()
            .into();
        assert!(value.starts_with("Downvote"));

        // comment tests
        let mut client = UserItemUnboundedReactionClient {
            store: &mut store,
            user: User("1000".into()),
            item: Item("2000".into()),
            reaction_type: PhantomData::<Comment>,
        };
        let rid = client.react(Comment("3000".into())).await.unwrap();

        let value: String = txn
            .get(Key::raw(format!("ui_Comment_User:1000_Item:2000_{}", rid)))
            .await
            .unwrap()
            .unwrap()
            .into();
        assert_eq!(&value, "Comment:3000");
        let value: String = txn
            .get(Key::raw("u_Comment_User:1000_Item:2000_Comment:3000"))
            .await
            .unwrap()
            .unwrap()
            .into();
        assert_eq!(value, rid);
    }
}
