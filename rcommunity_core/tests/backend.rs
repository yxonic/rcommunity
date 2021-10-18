use rcommunity_core::{
    backend::Backend, error::Error, ItemType, Multiple, ReactionType, Unique, UserType, WithData,
};

#[derive(Debug)]
struct MockBackend;
impl Backend<'_> for MockBackend {}

#[derive(Debug, Clone)]
struct User(String);
impl Unique for User {}
impl UserType for User {}

#[derive(Debug, Clone)]
struct Post(String);
impl Unique for Post {}
impl ItemType for Post {}

#[derive(Debug, Clone)]
struct Comment(String);
impl Unique for Comment {}
impl ItemType for Comment {}
impl WithData for Comment {
    type Item = String;
}
impl ReactionType for Comment {}
impl Multiple for Comment {}

#[tokio::test]
async fn test_backend() -> Result<(), Error> {
    MockBackend
        .push_reaction(
            &User("1000".into()),
            &Post("2000".into()),
            &Comment("3000".into()),
        )
        .await?;
    Ok(())
}
