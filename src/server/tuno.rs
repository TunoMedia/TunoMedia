use std::fs;
use log::trace;
use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("tuno");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("tuno_descriptor");
}

pub struct TunoService {}

#[tonic::async_trait]
impl proto::tuno_server::Tuno for TunoService {
    async fn echo(
        &self,
        request: Request<proto::EchoRequest>
    ) -> Result<Response<proto::EchoResponse>, Status> {
        let message = request.into_inner().message;
        trace!("Received echo request: {:?}", message);
        
        if message.is_empty() {
            return Err(Status::invalid_argument("Invalid echo request: message is empty"));
        }
        
        Ok(Response::new(proto::EchoResponse { message }))
    }
    
    async fn stream(
        &self,
        request: Request<proto::StreamRequest>
    ) -> Result<Response<proto::StreamResponse>, Status> {
        let object_id = request.into_inner().object_id;
        trace!("Received stream request: {:?}", object_id);
        
        if object_id.is_empty() {
            return Err(Status::invalid_argument("Invalid stream request: object_id is empty"));
        }
        
        match fs::read(format!("./media/{object_id}.mp3")) {
            Ok(data) => Ok(Response::new(proto::StreamResponse { data })),
            Err(_) => Err(Status::not_found(format!("Invalid object_id: {}", object_id)))
        }
    }
}