use service_protos::proto_file_service::{
    file_server::File, DeleteRequest, DeleteResponse, DownloadRequest, DownloadResponse,
    ListRequest, ListResponse, UploadRequest, UploadResponse,
};
use tonic::{Request, Response, Result, Status};

#[derive(Default, Debug)]
pub struct FileServer {}

#[async_trait::async_trait]
impl File for FileServer {
    async fn list(&self, _request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        todo!()
    }

    async fn upload(
        &self,
        _request: Request<UploadRequest>,
    ) -> Result<Response<UploadResponse>, Status> {
        todo!()
    }

    async fn download(
        &self,
        _request: Request<DownloadRequest>,
    ) -> Result<Response<DownloadResponse>, Status> {
        todo!()
    }

    async fn delete(
        &self,
        _request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        todo!()
    }
}
