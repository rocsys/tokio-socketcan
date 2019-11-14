use tokio_socketcan::CANSocket;
use futures_util::StreamExt;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut socket_rx = CANSocket::open("vcan0").unwrap();
    // let socket_tx = tokio_socketcan::CANSocket::open("vcan0").unwrap();

    println!("Reading and writing on vcan0");

    while let Some(next) = socket_rx.next().await {
        println!("{:#?}", next);
    }

    Ok(())
}
