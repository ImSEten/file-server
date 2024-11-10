use crate::Result;
// Q: request, S: response  todo! this designes are not good , may change it in feature
#[async_trait::async_trait]
pub trait ServerInterface<Q, S> {
    // fn list(&self, _request: Q) -> Result<S, E>;

    async fn start(&self) -> Result<()>;

    async fn stop(&self) -> Result<()>;

    async fn stats(&self) -> Result<()>;

    async fn request(&self, _request: Q) -> Result<S>;
}
