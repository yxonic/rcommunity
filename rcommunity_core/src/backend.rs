use async_trait::async_trait;

#[async_trait]
pub trait Backend<'backend> {
    async fn query(&self);
}
