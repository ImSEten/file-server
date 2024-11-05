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
    async fn list(&mut self, remote_dir: String) -> Result<(), E>;
    async fn upload_files(
        &mut self,
        local_files: Vec<String>,
        remote_dir: String,
        max_simultaneous: usize,
    ) -> Result<(), E>;
    async fn download_files(
        &mut self,
        remote_files: Vec<String>,
        local_dir: String,
        max_simultaneous: usize,
    ) -> Result<(), E>;
    async fn delete_files(&mut self, remote_files: Vec<String>) -> Result<(), E>;
    async fn move_files(
        &mut self,
        src_files: Vec<String>,
        destination_dir: String,
    ) -> Result<(), E>;
}
