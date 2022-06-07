use async_trait::async_trait;

use crate::{error::Result, store::Transaction};

use super::{ItemType, UserType};

#[async_trait]
pub trait Storable {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}
