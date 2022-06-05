use rcommunity_core::{
    backend::Backend, error::Error, query::UsersQuery, ItemType, Multiple, ReactionType, Unique,
    UserType, WithData,
};

#[derive(Debug)]
struct MockBackend;
impl Backend<'_> for MockBackend {}

#[derive(Debug, Clone)]
struct User(String);
impl<T> From<T> for User
where
    T: Into<String>,
{
    fn from(id: T) -> Self {
        User(id.into())
    }
}
impl Unique for User {}
impl UserType for User {}

#[derive(Debug, Clone)]
struct Post(String);
impl Unique for Post {}
impl ItemType for Post {}

#[derive(Debug, Clone)]
struct Comment(String);
impl<T> From<T> for Comment
where
    T: Into<String>,
{
    fn from(id: T) -> Self {
        Comment(id.into())
    }
}
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
    println!("{:?}", root.get("1000").item::<Comment>().given());
    assert!(root.get("1000").item::<Comment>().given().count().await? == 0);
    let r = root
        .get("1000")
        .item::<Comment>()
        .get("2000")
        .reaction::<Comment>();
    r.create("2001").with_data("hello").push().await;

    Ok(())
}
