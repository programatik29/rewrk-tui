use crate::state::State;
use pin_project::pin_project;
use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

#[pin_project]
pub struct RecordStream<IO> {
    #[pin]
    io: IO,
    state: Arc<State>,
}

impl<IO> RecordStream<IO> {
    pub fn new(io: IO, state: Arc<State>) -> Self {
        Self { io, state }
    }
}

impl<IO: AsyncRead> AsyncRead for RecordStream<IO> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.project();

        let old_length = buf.filled().len();
        let result = this.io.poll_read(cx, buf);
        let new_length = buf.filled().len();

        let bytes_read = new_length.saturating_sub(old_length);

        this.state.transfer_total.add(bytes_read as u64);

        result
    }
}

impl<IO: AsyncWrite> AsyncWrite for RecordStream<IO> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        self.project().io.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        self.project().io.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        self.project().io.poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        self.project().io.poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.io.is_write_vectored()
    }
}
