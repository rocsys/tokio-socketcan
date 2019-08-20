#![feature(async_await)]

use futures_util::stream::StreamExt;
use tokio_socketcan::CANSocket;

use futures::future::{self, Future};
use futures::stream::Stream;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut socket_rx = CANSocket::open("vcan0").unwrap();
    // let socket_tx = tokio_socketcan::CANSocket::open("vcan0").unwrap();

    println!("Reading and writing on vcan0");

    while let Some(next) = socket_rx.next().await {
        println!("{:#?}", next);
    }

    Ok(())
}
