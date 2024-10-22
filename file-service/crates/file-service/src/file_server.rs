use service_protos::proto_file_service::{
    file_server::File, DeleteFileRequest, DeleteFileResponse, DownloadFileRequest,
    DownloadFileResponse, ListRequest, ListResponse, UploadFileRequest, UploadFileResponse,
};
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
        _request: Request<UploadFileRequest>,
    ) -> Result<Response<UploadFileResponse>, Status> {
        todo!()
    }

    async fn download_file(
        &self,
        _request: Request<DownloadFileRequest>,
    ) -> Result<Response<DownloadFileResponse>, Status> {
        todo!()
    }

    async fn delete_file(
        &self,
        _request: Request<DeleteFileRequest>,
    ) -> Result<Response<DeleteFileResponse>, Status> {
        todo!()
    }
}
