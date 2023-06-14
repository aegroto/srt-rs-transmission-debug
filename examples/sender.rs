use bytes::Bytes;
use futures::stream;
use futures::{SinkExt, StreamExt};
use srt_tokio::SrtSocket;
use srt_tokio::options::ByteCount;
use std::io::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut srt_socket = SrtSocket::builder()
        .set(|options| {
            // Remove this option to freeze the transmission
            options.sender.buffer_size = ByteCount(1024 * 1024)
        })
        .listen_on(":3333").await?;

    let mut stream = stream::unfold(0, |count| async move {
        log::info!("Sent {count:?} packets");
        sleep(Duration::from_millis(10)).await;
        Some((Ok((Instant::now(), Bytes::from(vec![0; 64000]))), count + 1))
    })
    .boxed();

    srt_socket.send_all(&mut stream).await?;
    Ok(())
}
