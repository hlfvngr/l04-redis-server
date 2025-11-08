use std::io;

use anyhow::Result;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("listening on: {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("accepted: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream).await {
                warn!("process occured err: {:?}", e)
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn process_redis_conn(mut stream: TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(4096);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(_len) => {
                info!("{:?}", String::from_utf8_lossy(&buf));
                let _ = stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                warn!("occured err: {}", e);
                return Err(e.into());
            }
        };
    }
    Ok(())
}
