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

    pub async fn upload_file(&mut self) -> Result<(), Status> {
        let request = UploadFileRequest {};
        if let Some(client) = self.client.as_mut() {
            match client.upload_file(request).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "clien is None").into())
        }
    }

    pub async fn download_file(&mut self) -> Result<(), Status> {
        let request = DownloadFileRequest {};
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
        let request = DeleteFileRequest {};
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
