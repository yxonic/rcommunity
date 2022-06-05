use std::marker::PhantomData;

use crate::{
    error::Result,
    markers::{ItemType, ReactionType, UserType},
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
    store: &'store TS,
    user: TU,
    item: TI,
    reaction_type: PhantomData<TR>,
}

impl<'store, TS: Store, TU: UserType, TI: ItemType, TR: ReactionType>
    UserItemUnboundedReactionClient<'store, TS, TU, TI, TR>
{
    async fn push(&mut self, reaction: impl Into<TR>) -> Result<()> {
        let mut txn = self.store.txn_begin().await?;
        txn.put("".into(), "".into()).await?;
        txn.commit().await
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::{
        markers::{ItemType, ReactionType, UserType},
        store::memory::MemoryStore,
    };

    use super::UserItemUnboundedReactionClient;

    // test types
    struct User(String);
    impl UserType for User {}
    struct Item(String);
    impl ItemType for Item {}
    struct Comment(String);
    impl ReactionType for Comment {}
    impl ItemType for Comment {}

    #[tokio::test]
    async fn test_reaction() {
        let store = MemoryStore::default();
        let mut client = UserItemUnboundedReactionClient {
            store: &store,
            user: User("1000".into()),
            item: Item("2000".into()),
            reaction_type: PhantomData::<Comment>,
        };
        client.push(Comment("3000".into())).await.unwrap();
    }
}
