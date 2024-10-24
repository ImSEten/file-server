use service_protos::proto_file_service::{
    file_server::File, DeleteFileRequest, DeleteFileResponse, DownloadFileRequest,
    DownloadFileResponse, ListRequest, ListResponse, UploadFileRequest, UploadFileResponse,
};
use tokio::io::AsyncWriteExt;
use tonic::{Request, Response, Result, Status};

#[derive(Default, Debug)]
pub struct FileServer {}

#[async_trait::async_trait]
impl File for FileServer {
    async fn list(&self, _request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        todo!()
    }

    async fn upload_file(
        &self,
        request: Request<tonic::Streaming<UploadFileRequest>>,
    ) -> Result<Response<UploadFileResponse>, Status> {
        let message = request.into_inner().message().await?;
        if let Some(upload_file_request) = message {
            let file = std::path::PathBuf::from(upload_file_request.file_path.clone())
                .join(upload_file_request.file_name.clone());
            let mut f = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(file)
                .await?;
            let _ = f.write_all(&upload_file_request.content).await?;
            f.flush().await?;
            let upload_file_response = UploadFileResponse {
                file_name: upload_file_request.file_name,
                file_path: upload_file_request.file_path,
            };
            Ok(tonic::Response::new(upload_file_response))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "request is None").into())
        }
    }

    async fn download_file(
        &self,
        _request: Request<DownloadFileRequest>,
    ) -> Result<Response<futures::stream::BoxStream<'static, Result<DownloadFileResponse>>>, Status>
    {
        todo!()
    }

    async fn delete_file(
        &self,
        _request: Request<DeleteFileRequest>,
    ) -> Result<Response<DeleteFileResponse>, Status> {
        todo!()
    }
}
