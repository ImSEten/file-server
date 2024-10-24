use tokio::io::AsyncReadExt;
use tonic::{transport::Channel, Response, Status};

use service_protos::proto_file_service::{
    file_client::FileClient, DeleteFileRequest, DownloadFileRequest, ListRequest, ListResponse,
    UploadFileRequest,
};

#[derive(Default, Debug, Clone)]
pub struct Client {
    pub server_ip: String,
    pub port: String,
    pub client: Option<FileClient<Channel>>,
}

impl Client {
    pub async fn new(server_ip: String, port: String) -> Self {
        Client {
            server_ip: server_ip.clone(),
            port: port.clone(),
            client: FileClient::connect(
                "http://".to_string() + server_ip.as_str() + ":" + port.as_str(),
            )
            .await
            .ok(),
        }
    }

    pub async fn list(&mut self) -> Result<Response<ListResponse>, Status> {
        let client = self.client.as_mut().ok_or(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "client is None",
        ))?;
        client.list(ListRequest::default()).await
    }

    pub async fn upload_file(
        &mut self,
        local_file: String,
        remote_dir: String,
    ) -> Result<(), Status> {
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
        let mut request = UploadFileRequest {
            file_name,
            file_path: remote_dir,
            content: Vec::new(),
        };
        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .open(local_file)
            .await?;
        f.read_to_end(&mut request.content).await?;
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<UploadFileRequest>();
        sender
            .send(request)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let receiver_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(receiver);
        if let Some(client) = self.client.as_mut() {
            match client.upload_file(receiver_stream).await {
                Ok(_) => Ok(()),
                // todo: if returned error is exist, we need to ask for the user whether truncate the file or create a new one.
                Err(e) => Err(e),
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "clien is None").into())
        }
    }

    pub async fn download_file(&mut self) -> Result<(), Status> {
        let request = DownloadFileRequest {
            file_name: "test".to_string(),
            file_path: "test".to_string(),
        };
        if let Some(client) = self.client.as_mut() {
            match client.download_file(request).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "clien is None").into())
        }
    }

    pub async fn delete_file(&mut self) -> Result<(), Status> {
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

    pub async fn re_connect(&mut self) -> Result<(), Status> {
        self.client = FileClient::connect(
            "http://".to_string() + self.server_ip.as_str() + ":" + self.port.as_str(),
        )
        .await
        .ok();
        if self.client.is_none() {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "clien is None").into())
        } else {
            Ok(())
        }
    }
}
