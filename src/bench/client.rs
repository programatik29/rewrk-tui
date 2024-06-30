use super::{error::BenchResult, record::RecordStream};
use crate::{
    args::{Args, Target},
    bench::error::BenchError,
    state::State,
};
use http_body_util::{BodyExt, Empty};
use hyper::{body::Bytes, client::conn::http1::SendRequest, header::HOST, Request};
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct Client {
    request: Request<Empty<Bytes>>,
    sender: SendRequest<Empty<Bytes>>,
}

impl Client {
    pub async fn connect(args: &Args, state: &Arc<State>) -> BenchResult<Self> {
        let Target {
            ref host,
            port,
            ref path,
        } = args.target;

        let stream = TcpStream::connect((host.as_str(), port))
            .await
            .map_err(BenchError::conn_failed)?;
        let io = TokioIo::new(RecordStream::new(stream, state.clone()));

        let host = match port {
            80 => host.clone(),
            port => format!("{host}:{port}"),
        };
        let request = Request::get(path)
            .header(HOST, host)
            .body(Empty::<Bytes>::new())
            .unwrap();

        let (sender, connection) = hyper::client::conn::http1::handshake(io)
            .await
            .map_err(BenchError::conn_failed)?;

        tokio::spawn(async {
            let _ = connection.await;
        });

        Ok(Self { request, sender })
    }

    pub async fn send_request(&mut self) -> BenchResult<()> {
        self.sender.ready().await.map_err(BenchError::conn_closed)?;

        let response = self
            .sender
            .send_request(self.request.clone())
            .await
            .map_err(BenchError::conn_closed)?;

        response.collect().await.map_err(BenchError::conn_closed)?;

        Ok(())
    }
}
