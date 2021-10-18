use rcommunity_core::{
    backend::Backend, error::Error, query::UsersQuery, ItemType, Multiple, ReactionType, Unique,
    UserType, WithData,
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
impl ReactionType for Comment {}
impl Multiple for Comment {}
impl WithData for Comment {
    type Item = String;
}

#[tokio::test]
async fn test_query() -> Result<(), Error> {
    let root: UsersQuery<MockBackend, User> = UsersQuery::new(&MockBackend);
    println!(
        "{:?}",
        root.get(User("1000".into())).item::<Comment>().given()
    );
    assert!(
        root.get(User("1000".into()))
            .item::<Comment>()
            .given()
            .count()
            .await?
            == 0
    );
    let r = root
        .get(User("1000".into()))
        .item::<Comment>()
        .get(Comment("2000".into()))
        .reaction::<Comment>();
    r.create(Comment("2001".into()))
        .with_data("hello")
        .push()
        .await;

    Ok(())
}
