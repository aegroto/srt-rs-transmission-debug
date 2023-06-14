use srt_tokio::{SrtSocket, options::ByteCount};
use std::io::Error;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut srt_socket = SrtSocket::builder()
        .set(|options| {
            options.receiver.buffer_size = ByteCount(1024 * 1024)
        })
        .call("127.0.0.1:3333", None).await?;
    let mut count = 0;

    while let Some((_instant, _bytes)) = srt_socket.try_next().await? {
        count += 1;
        log::info!("Received {count:?} packets");
    }

    println!("\nConnection closed");

    Ok(())
}