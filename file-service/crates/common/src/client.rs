use std::error::Error;

#[async_trait::async_trait]
pub trait Client<E>
where
    E: Error + Send + Sync,
{
    async fn new(server_ip: String, port: String) -> Self
    where
        Self: Send + Sync;
    // todo: list return should not be ()
    async fn list(&mut self) -> Result<(), E>;
    async fn upload_files(
        &mut self,
        local_file: Vec<String>,
        remote_dir: String,
        max_simultaneous_uploads: u16,
    ) -> Result<(), E>;
    async fn download_file(&mut self) -> Result<(), E>;
    async fn delete_file(&mut self) -> Result<(), E>;
}
