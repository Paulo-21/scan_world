use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpSocket;
use tokio::time::{timeout, Duration};

pub async fn job(ip : u32) {
    /*let one = (ip >> 24) as u8 & 255 ;
    //let one = 86;
    let two = (ip >> 16 ) as u8 & 255;
    let tree = (ip >> 8) as u8 & 255;
    let four = (ip & 255) as u8;*/
    //println!("{} {} {} {}", one, two, tree, four);
    
    //let ip = IpAddr::V4(Ipv4Addr::new(one, two, tree, four));
    let ip = IpAddr::V4(Ipv4Addr::from(ip));
    let addr = SocketAddr::new(ip, 22);
    let socket = TcpSocket::new_v4().unwrap();
    let one_sec = Duration::from_secs(1);
    socket.set_reuseport(true).unwrap();
    match timeout(one_sec, socket.connect(addr)).await {
        Ok(res) => {
            match res {
                Ok(_stream) => {
                    println!("CONNECTED {:?}", addr);
                },
                Err(_err) => {
                    //println!("err {}", err);
                }
            }
        },
        Err(_err) => {
            //println!("TO : {}", err);
        }
    }
}
