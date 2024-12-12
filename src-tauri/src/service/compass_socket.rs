use crate::model::compass::Compass;
use tokio::net::UdpSocket;

pub async fn socket_init(bind_addr: &str) -> tokio::io::Result<UdpSocket> {
    let socket = UdpSocket::bind(bind_addr).await?;
    socket.set_broadcast(false)?;
    // println!("Socket initialized at {}", bind_addr);
    Ok(socket)
}

pub async fn receive_data(socket: &UdpSocket) -> Option<Compass> {
    let mut buf = [0; 2048];
    match socket.recv_from(&mut buf).await {
        Ok((size, src)) => {
            let received_data: Compass = serde_json::from_slice(&buf[..size]).unwrap();
            // println!("Received from {}: {:#?}", src, received_data);
            Some(received_data)
        }
        Err(e) => {
            eprintln!("Error receiving data: {}:{}: {}", file!(), line!(), e);
            None
        }
    }
}
