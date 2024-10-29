use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonic::{transport::Channel, Status};

use service_protos::proto_file_service::{
    grpc_file_client::GrpcFileClient, DeleteFileRequest, DownloadFileRequest, ListRequest, UploadFileRequest,
};

use common::{client, file};

#[derive(Default, Debug, Clone)]
pub struct GRPCClient {
    pub server_ip: String,
    pub port: String,
    pub client: Option<GrpcFileClient<Channel>>,
}

impl GRPCClient {
    async fn upload_file(&mut self, local_file: String, remote_dir: String) -> Result<(), Status> {
        let file_name;
        if let Some(file_name_str) = std::path::PathBuf::from(local_file.clone()).file_name() {
            if let Some(f) = file_name_str.to_str() {
                file_name = f.to_string();
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "file name contains non-UTF-8 charactors",
                )
                .into());
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "no file name found in the path",
            )
            .into());
        }
        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .open(local_file)
            .await?;

        let (sender, receiver) = tokio::sync::mpsc::channel::<UploadFileRequest>(1);
        let handle = tokio::spawn(async move {
            loop {
                let mut request = UploadFileRequest {
                    file_name: file_name.clone(),
                    file_path: remote_dir.clone(),
                    content: Vec::with_capacity(1024 * 1024),
                };
                if let Ok(lens) = f.read_buf(&mut request.content).await {
                    if lens == 0 {
                        break; //EOF
                    }

                    match sender
                        .send(request)
                        .await
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                    {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                } else {
                    break;
                }
            }
            Ok(())
        });

        let receiver_stream = tokio_stream::wrappers::ReceiverStream::new(receiver);
        if let Some(client) = self.client.as_mut() {
            match client.upload_file(receiver_stream).await {
                Ok(_) => {}
                // todo: if returned error is exist, we need to ask for the user whether truncate the file or create a new one.
                Err(e) => return Err(e),
            }
            let result = tokio::join!(handle).0;
            Ok(result.unwrap()?)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "client is None").into())
        }
    }

    async fn download_file(
        &mut self,
        remote_file: String,
        local_dir: String,
    ) -> Result<(), Status> {
        let path_buf = std::path::Path::new(&remote_file);
        let file_path = file::get_file_parent(path_buf)?;
        let file_name = file::get_file_name(path_buf)?;
        let download_request = DownloadFileRequest {
            file_name: file_name.clone(),
            file_path: file_path.clone(),
        };
        if let Some(client) = self.client.as_mut() {
            match client.download_file(download_request).await {
                Ok(stream_response) => {
                    let mut stream = stream_response.into_inner();
                    let file = std::path::PathBuf::from(local_dir.clone()).join(file_name.clone());
                    if file.exists() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::AlreadyExists,
                            format!("file {} already exists", file.to_str().unwrap()),
                        )
                        .into());
                    }
                    let mut f = tokio::fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(file)
                        .await?;
                    let mut write_times = 0;
                    while let Some(download_file_response) = stream.message().await? {
                        let len = f.write(&download_file_response.content).await?;
                        write_times += 1;
                        // Reduce the number of flushes and protect disks.
                        // Here the disk is written every 100 MB.
                        if write_times % 100 == 0 {
                            f.flush().await?;
                        }
                        if len == 0 {
                            break;
                        }
                    }
                    f.flush().await?;
                }
                // todo: if returned error is exist, we need to ask for the user whether truncate the file or create a new one.
                Err(e) => return Err(e),
            }
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "client is None").into())
        }
    }
}

#[async_trait::async_trait]
impl client::Client<Status> for GRPCClient {
    async fn new(server_ip: String, port: String) -> Self {
        GRPCClient {
            server_ip: server_ip.clone(),
            port: port.clone(),
            client: GrpcFileClient::connect(
                "http://".to_string() + server_ip.as_str() + ":" + port.as_str(),
            )
            .await
            .ok(),
        }
    }

    async fn list(&mut self) -> Result<(), Status> {
        let client = self.client.as_mut().ok_or(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "client is None",
        ))?;
        client.list(ListRequest::default()).await?;
        Ok(())
    }

    async fn upload_files(
        &mut self,
        local_files: Vec<String>,
        remote_dir: String,
        _max_simultaneous_uploads: u16,
    ) -> Result<(), Status> {
        // todo: to spawn to upload, upload the max_simultaneous_uploads at the same time.
        for local_file in local_files {
            self.upload_file(local_file, remote_dir.clone()).await?;
        }
        Ok(())
    }

    async fn download_files(
        &mut self,
        remote_files: Vec<String>,
        local_dir: String,
    ) -> Result<(), Status> {
        for remote_file in remote_files {
            self.download_file(remote_file, local_dir.clone()).await?;
        }
        Ok(())
    }

    async fn delete_file(&mut self) -> Result<(), Status> {
        let request = DeleteFileRequest {
            file_name: "".to_string(),
            file_path: "test".to_string(),
        };
        if let Some(client) = self.client.as_mut() {
            match client.delete_file(request).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "clien is None").into())
        }
    }
}
