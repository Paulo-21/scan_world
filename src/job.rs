use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::{io, net::TcpSocket};

pub async fn job(ip : u32) -> io::Result<()> {
    let one = (ip >> 24) as u8 & 255 ;
    let two = (ip >> 16 ) as u8 & 255;
    let tree = (ip >> 8) as u8 & 255;
    let four = (ip & 255) as u8;
    //println!("{} {} {} {}", one, two, tree, four);
    let socket = TcpSocket::new_v4()?;
    let ip = IpAddr::V4(Ipv4Addr::new(one, two, tree, four));
    let addr = SocketAddr::new(ip, 8080);
    let stream = socket.connect(addr).await?;

    Ok(())
}