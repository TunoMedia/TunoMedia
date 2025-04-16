use std::{io::Read, pin::Pin};
use log::{error, trace};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Status};

use iota_sdk::types::base_types::ObjectID;

use crate::{local_storage::get_local_song_reader, server::utils::verify_payment};

pub mod pb {
    tonic::include_proto!("tuno");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("tuno_descriptor");
}

pub(crate) struct TunoService {
    package_id: ObjectID
}

impl TunoService {
    pub(crate) fn new(package_id: ObjectID) -> Self {
        Self { package_id }
    }
}

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
        let song_id = match verify_payment(request.into_inner().raw_transaction, self.package_id) {
            Ok(song_id) => song_id,
            Err(e) => {
                error!("Error verifying tx: {e}");
                return Err(Status::permission_denied("Transaction could not be verified"));
            }
        };

        let mut reader = match get_local_song_reader(&song_id) {
            Ok(reader) => reader,
            Err(e) => {
                error!("Error while seeking {song_id}: {e}");
                return Err(Status::not_found(format!("Unknown object_id: {song_id}")));
            }
        };

        let mut data = vec![];
        match reader.read_to_end(&mut data) {
            Ok(_) => {
                trace!("Succesful fetch request for {song_id}");
                Ok(Response::new(pb::SongBytes { data }))
            },
            Err(e) => {
                error!("Error reading {song_id}: {e}");
                Err(Status::not_found(format!("Invalid object_id: {}", song_id)))
            }
        }
    }

    async fn stream_song(
        &self,
        request: Request<pb::SongStreamRequest>
    ) -> Result<Response<Self::StreamSongStream>, Status> {
        let song_stream_request = request.into_inner();
        let Some(pb::SongRequest { raw_transaction }) = song_stream_request.req else {
            error!("req parameter not found");
            return Err(Status::invalid_argument("req"));
        };
        
        let song_id = match verify_payment(raw_transaction, self.package_id) {
            Ok(song_id) => song_id,
            Err(e) => {
                error!("Error verifying tx: {e}");
                return Err(Status::permission_denied("Transaction could not be verified"));
            }
        };

        let mut reader = match get_local_song_reader(&song_id) {
            Ok(reader) => reader,
            Err(e) => {
                error!("Error while seeking {song_id}: {e}");
                return Err(Status::not_found(format!("Unknown object_id: {song_id}")));
            }
        };

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut buf = vec![0; song_stream_request.block_size as usize];

            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 { break }
                match tx.send(Ok(pb::SongBytes { data: buf[..n].to_vec() })).await {
                    Ok(_) => continue,
                    Err(e) => {
                        error!("Error while streaming: {e}");
                        break
                    }
                }
            }
        });

        trace!("Finished stream request for {song_id}");
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}