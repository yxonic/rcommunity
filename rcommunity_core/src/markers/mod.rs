use async_trait::async_trait;

use crate::{error::Result, store::Transaction};

pub mod store;

pub trait UserType: Sync {}
pub trait ItemType: Sync {}
pub trait ReactionType: Sync {}

#[async_trait]
impl<T: ReactionType> store::Storable for T {
    default async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        println!("store reaction");
        Ok(())
    }
    default async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        Ok(())
    }
}

pub trait Unique {}

#[async_trait]
impl<T: ReactionType + Unique> store::Storable for T {
    default async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        println!("store unique index");
        Ok(())
    }
}
