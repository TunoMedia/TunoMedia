use std::{io::Read, pin::Pin};
use log::trace;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Status};

use crate::server::utils::get_song_reader;

pub mod pb {
    tonic::include_proto!("tuno");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("tuno_descriptor");
}

pub struct TunoService {}

#[tonic::async_trait]
impl pb::tuno_server::Tuno for TunoService {
    type StreamSongStream = Pin<Box<dyn Stream<Item = Result<pb::SongBytes, Status>> + Send>>;

    async fn echo(
        &self,
        request: Request<pb::EchoRequest>
    ) -> Result<Response<pb::EchoResponse>, Status> {
        let message = request.into_inner().message;
        trace!("Received echo request: {:?}", message);
        
        if message.is_empty() {
            return Err(Status::invalid_argument("Invalid echo request: message is empty"));
        }
        
        Ok(Response::new(pb::EchoResponse { message }))
    }

    async fn fetch_song(
        &self,
        request: Request<pb::SongRequest>
    ) -> Result<Response<pb::SongBytes>, Status> {
        let object_id = request.into_inner().object_id;
        trace!("Received fetch request: {:?}", object_id);
        
        let mut data = vec![];
        if let Ok(mut reader) = get_song_reader(&object_id) {
            if reader.read_to_end(&mut data).is_ok() {
                return Ok(Response::new(pb::SongBytes { data }));
            } 
        }
        
        Err(Status::not_found(format!("Invalid object_id: {}", object_id)))
    }

    async fn stream_song(
        &self,
        request: Request<pb::SongStreamRequest>
    ) -> Result<Response<Self::StreamSongStream>, Status> {
        let song_stream_request = request.into_inner();
        let object_id = song_stream_request.object_id;
        trace!("Received stream request: {:?}", object_id);

        let mut reader = match get_song_reader(&object_id) {
            Ok(reader) => reader,
            Err(_) => return Err(Status::not_found(format!("Invalid object_id: {}", object_id)))
        };

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut buf = vec![0; song_stream_request.block_size as usize];

            trace!("Streaming song: {:?}", object_id);
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 { break }
                if tx.send(Ok(pb::SongBytes { data: buf[..n].to_vec() })).await.is_err() {
                    break;
                }
            }

            trace!("Done!");
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}