use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    markers::{ItemType, Once, ReactionType, UserType},
    store::{
        format::{from_value, to_key, to_value, TypeName},
        Transaction,
    },
};

#[derive(Serialize)]
#[serde(rename = "ReactionInfoKey")]
pub struct ReactionInfoKeyRef<'a, TR>
where
    TR: ReactionType,
{
    pub reaction_type: TypeName<TR>,
    pub rid: &'a str,
}

#[derive(Serialize)]
#[serde(rename = "ReactionInfoValue")]
pub struct ReactionInfoValueRef<'a, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub user: &'a TU,
    pub item: &'a TI,
    pub reaction: &'a TR,
}

#[derive(Deserialize)]
pub struct ReactionInfoValue<TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub user: TU,
    pub item: TI,
    pub reaction: TR,
}

#[derive(Serialize)]
#[serde(rename = "UserItemToReactionKey")]
pub struct UserItemToReactionKeyRef<'a, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub reaction_type: TypeName<TR>,
    pub user: &'a TU,
    pub item: &'a TI,
    pub rid: &'a str,
}

#[derive(Serialize)]
#[serde(rename = "UserItemToReactionValue")]
pub(crate) struct UserItemToReactionValueRef<'a, TR>
where
    TR: ReactionType,
{
    pub reaction: &'a TR,
}

#[derive(Serialize)]
#[serde(rename = "UserItemToReactionOnceKey")]
pub struct UserItemToReactionOnceKeyRef<'a, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub reaction_type: TypeName<TR>,
    pub user: &'a TU,
    pub item: &'a TI,
}

#[derive(Serialize)]
#[serde(rename = "UserItemToReactionOnceValue")]
pub(crate) struct UserItemToReactionOnceValueRef<'a> {
    pub rid: &'a str,
}

#[derive(Deserialize)]
pub(crate) struct UserItemToReactionOnceValue {
    pub rid: String,
}

/// Ability to manage and query reaction basic information.
#[async_trait]
pub trait ReactionInfo: ReactionType {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn get_reaction_by_id<TU, TI>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<ReactionInfoValue<TU, TI, Self>>
    where
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned;
}

/// Default [`ReactionInfo`] implementor for all reaction types.
///
/// Under the hood, this implementor manages **reaction ID** to
/// **user-item-reaction triplet** mapping for all reaction types.
#[async_trait]
impl<T: ReactionType + DeserializeOwned> ReactionInfo for T {
    default async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let key = ReactionInfoKeyRef {
            reaction_type: TypeName::<T>::new(),
            rid,
        };
        let value = ReactionInfoValueRef {
            user,
            item,
            reaction: self,
        };
        txn.put(
            &to_key(&key).map_err(Error::SerializationError)?,
            &to_value(&value).map_err(Error::SerializationError)?,
        )
        .await?;
        let key = UserItemToReactionKeyRef {
            reaction_type: TypeName::<T>::new(),
            user,
            item,
            rid,
        };
        let value = UserItemToReactionValueRef { reaction: self };
        txn.put(
            &to_key(&key).map_err(Error::SerializationError)?,
            &to_value(&value).map_err(Error::SerializationError)?,
        )
        .await?;
        Ok(())
    }
    default async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let key = ReactionInfoKeyRef {
            reaction_type: TypeName::<T>::new(),
            rid,
        };
        txn.delete(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;

        let key = UserItemToReactionKeyRef {
            reaction_type: TypeName::<T>::new(),
            user,
            item,
            rid,
        };
        txn.delete(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;
        Ok(())
    }
    default async fn get_reaction_by_id<TU, TI>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<ReactionInfoValue<TU, TI, T>>
    where
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned,
    {
        let key = ReactionInfoKeyRef {
            reaction_type: TypeName::<T>::new(),
            rid,
        };
        let value = txn
            .get(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;
        if let Some(v) = value {
            return from_value(&v).map_err(Error::SerializationError);
        }
        Err(Error::UnknownError("TODO: change to not found".into()))
    }
}

/// Specialized [`ReactionInfo`] implementor for reaction types that are marked
/// as [`Once`].
///
/// Under the hood, this implementor manages **reaction ID** to **user-item
/// pair** mapping for reaction types that react at most once for each user-item
/// pair.
#[async_trait]
impl<T: ReactionType + DeserializeOwned + Once> ReactionInfo for T {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let key = ReactionInfoKeyRef {
            reaction_type: TypeName::<T>::new(),
            rid,
        };
        let value = ReactionInfoValueRef {
            user,
            item,
            reaction: self,
        };
        txn.put(
            &to_key(&key).map_err(Error::SerializationError)?,
            &to_value(&value).map_err(Error::SerializationError)?,
        )
        .await?;
        let key = UserItemToReactionOnceKeyRef {
            reaction_type: TypeName::<T>::new(),
            user,
            item,
        };
        let value = UserItemToReactionOnceValueRef { rid };
        txn.put(
            &to_key(&key).map_err(Error::SerializationError)?,
            &to_value(&value).map_err(Error::SerializationError)?,
        )
        .await?;
        Ok(())
    }
    async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let key = ReactionInfoKeyRef {
            reaction_type: TypeName::<T>::new(),
            rid,
        };
        txn.delete(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;

        let key = UserItemToReactionOnceKeyRef {
            reaction_type: TypeName::<T>::new(),
            user,
            item,
        };
        txn.delete(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait ReactionInfoOnce: ReactionType {
    async fn get_rid<TU, TI>(txn: &mut impl Transaction, user: &TU, item: &TI) -> Result<String>
    where
        TU: UserType,
        TI: ItemType;
}

#[async_trait]
impl<T: ReactionType + Once> ReactionInfoOnce for T {
    async fn get_rid<TU, TI>(txn: &mut impl Transaction, user: &TU, item: &TI) -> Result<String>
    where
        TU: UserType,
        TI: ItemType,
    {
        let key = UserItemToReactionOnceKeyRef {
            reaction_type: TypeName::<T>::new(),
            user,
            item,
        };
        let value = txn
            .get(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;
        if let Some(v) = value {
            let v: UserItemToReactionOnceValue =
                from_value(&v).map_err(Error::SerializationError)?;
            Ok(v.rid)
        } else {
            Err(Error::UnknownError("TODO: change to not found".into()))
        }
    }
}
