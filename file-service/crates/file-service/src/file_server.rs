use futures::StreamExt;
use service_protos::proto_file_service::{
    file_server::File, DeleteFileRequest, DeleteFileResponse, DownloadFileRequest,
    DownloadFileResponse, ListRequest, ListResponse, UploadFileRequest, UploadFileResponse,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonic::{Request, Response, Result, Status};

use common::file;

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
        // todo: if file is existed, we should return an exist error.
        let mut stream = request.into_inner();
        let upload_file_request =
            stream
                .message()
                .await?
                .ok_or(Status::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "requset is None",
                )))?;
        let file_path = upload_file_request.file_path;
        let file_name = upload_file_request.file_name;
        let file = std::path::PathBuf::from(file_path.clone()).join(file_name.clone());
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
        let _ = f.write(&upload_file_request.content).await?;
        #[allow(unused_variables)]
        let mut write_times: u32 = 1;

        while let Some(upload_file_request) = stream.message().await? {
            let len = f.write(&upload_file_request.content).await?;
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
        let upload_file_response = UploadFileResponse {
            file_name,
            file_path,
        };
        Ok(tonic::Response::new(upload_file_response))
    }

    async fn download_file(
        &self,
        request: Request<DownloadFileRequest>,
    ) -> Result<Response<futures::stream::BoxStream<'static, Result<DownloadFileResponse>>>, Status>
    {
        let req = request.into_inner();
        let file = std::path::Path::new(&req.file_path).join(&req.file_name);
        let file_parent = file::get_file_parent(&file)?;
        let file_name = file::get_file_name(&file)?;
        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .open(file.clone())
            .await?;

        let (sender, receiver) =
            tokio::sync::mpsc::channel::<Result<DownloadFileResponse, Status>>(1);
        let stream = tokio_stream::wrappers::ReceiverStream::new(receiver).boxed();
        tokio::spawn(async move {
            loop {
                let mut response = DownloadFileResponse {
                    file_name: file_name.clone(),
                    file_path: file_parent.clone(),
                    content: Vec::with_capacity(1024 * 1024),
                };
                if let Ok(lens) = f.read_buf(&mut response.content).await {
                    if lens == 0 {
                        break; //EOF
                    }
                    match sender
                        .send(Ok(response))
                        .await
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                    {
                        Ok(_) => {}
                        Err(e) => {
                            let error = e.to_string();
                            sender.send(Err(e.into())).await.unwrap_or_default();
                            return Err(std::io::Error::other(error));
                        }
                    }
                } else {
                    sender
                        .send(Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("read {:?} got error", file),
                        )
                        .into()))
                        .await
                        .unwrap_or_default();
                    break;
                }
            }
            Ok(())
        });
        Ok(Response::new(stream))
    }

    async fn delete_file(
        &self,
        _request: Request<DeleteFileRequest>,
    ) -> Result<Response<DeleteFileResponse>, Status> {
        todo!()
    }
}
