use std::time::Duration;

use std::io;

use async_std::task;
use async_std::net::{UdpSocket,SocketAddr};
use async_std::prelude::*;

use byte_string::ByteStr;

use snmp::SnmpPdu;

use thiserror::Error;

// use async_listen::{ListenExt, ByteStream, backpressure, error_hint};

const MIN_PDU_SIZE: i32 = 484;
#[derive(Error, Debug)]
pub enum SnmpAgentError {
    #[error("whoops, network")]
    Network(#[from] io::Error),
}

pub fn run(port : u16) {
    println!("SNMP Agent running");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    if let Err(e) = task::block_on(listen(addr)) {
        eprintln!("Error: {}", e);       
    }

}

async fn listen(address: SocketAddr) -> Result<(), SnmpAgentError> {
    println!("Binding to port {}", address);
    let socket = UdpSocket::bind(address).await?;
    let mut buf = vec![0; 1024];

    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", n, peer);

        let bytes = ByteStr::new(&buf[..n]);

        // Trace bytes in
        println!("Bytes {:?}", bytes);

        let pdu = SnmpPdu::from_bytes(bytes);

        // Trace PDU
        println!("Pdu {:?}", pdu);

        // simple echo
        let sent = socket.send_to(bytes, peer).await?;
        println!("Sent {} bytes to {}", sent, peer);
    }
    // Ok(()) 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
