// Q: request, S: response, E: error
#[async_trait::async_trait]
pub trait Server<Q, S, E> {
    fn list(&self, _request: Q) -> Result<S, E>;
}
