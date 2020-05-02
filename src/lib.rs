use std::time::Duration;

use std::io;

use async_std::task;
use async_std::net::TcpListener;
use async_std::prelude::*;

use async_listen::{ListenExt, ByteStream, backpressure, error_hint};

pub fn run() {
    println!("SNMP Agent running");
    if let Err(e) = task::block_on(listen()) {
        eprintln!("Error: {}", e);       
    }

}

async fn listen() -> Result<(), io::Error> {
    let (_, bp) = backpressure::new(10);
    let listener = TcpListener::bind("localhost:8080").await?;
    eprintln!("Accepting connections on localhost:8080");
    let mut incoming = listener.incoming()
        .log_warnings(log_accept_error)
        .handle_errors(Duration::from_millis(500))
        .backpressure_wrapper(bp);
    while let Some(stream) = incoming.next().await {
        task::spawn(async {
            if let Err(e) = connection_loop(stream).await {
                eprintln!("Error: {}", e);
            }
        });
    }
    Ok(())
}

async fn connection_loop(mut stream: ByteStream) -> Result<(), io::Error> {
    println!("Connected from {}", stream.peer_addr()?);
    task::sleep(Duration::from_secs(5)).await;
    stream.write_all("hello\n".as_bytes()).await?;
    Ok(())
}

fn log_accept_error(e: &io::Error) {
    eprintln!("Accept error: {}. Sleeping 0.5s. {}", e, error_hint(&e));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
