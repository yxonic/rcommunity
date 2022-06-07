use std::marker::PhantomData;

use crate::{
    error::Result,
    markers::{store::Storable, ItemType, ReactionType, UserType},
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
    async fn push(&mut self, reaction: impl Into<TR>) -> Result<()> {
        let mut txn = self.store.begin_txn().await?;
        let r = reaction.into();

        r.store_reaction(&mut txn, &self.user, &self.item).await?;
        r.store_unique_index(&mut txn, &self.user, &self.item)
            .await?;

        txn.commit().await
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::{
        markers::{ItemType, ReactionType, Unique, UserType},
        store::memory::MemoryStore,
    };

    use super::UserItemUnboundedReactionClient;

    // test types
    struct User(String);
    impl UserType for User {}
    struct Item(String);
    impl ItemType for Item {}

    enum Vote {
        Upvote,
        Downvote,
    }
    impl ReactionType for Vote {}
    impl ItemType for Vote {}

    struct Comment(String);
    impl ReactionType for Comment {}
    impl ItemType for Comment {}
    impl Unique for Comment {}

    #[tokio::test]
    async fn test_reaction() {
        let mut store = MemoryStore::default();
        let mut client = UserItemUnboundedReactionClient {
            store: &mut store,
            user: User("1000".into()),
            item: Item("2000".into()),
            reaction_type: PhantomData::<Vote>,
        };
        client.push(Vote::Upvote).await.unwrap();

        let mut client = UserItemUnboundedReactionClient {
            store: &mut store,
            user: User("1000".into()),
            item: Item("2000".into()),
            reaction_type: PhantomData::<Comment>,
        };
        client.push(Comment("3000".into())).await.unwrap();
    }
}
