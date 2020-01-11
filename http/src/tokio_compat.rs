use futures03::io::{AsyncRead, AsyncWrite};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct TokioCompat<T>(pub T);

impl<T: AsyncRead + Unpin> tokio::io::AsyncRead for TokioCompat<T> {
	fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize, io::Error>> {
		Pin::new(&mut self.0).poll_read(cx, buf)
	}
}

impl<T: AsyncWrite + Unpin> tokio::io::AsyncWrite for TokioCompat<T> {
	fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize, io::Error>> {
		Pin::new(&mut self.0).poll_write(cx, buf)
	}

	fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
		Pin::new(&mut self.0).poll_flush(cx)
	}

	fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
		Pin::new(&mut self.0).poll_close(cx)
	}
}
