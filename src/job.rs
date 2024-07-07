use tokio::{io, net::TcpSocket};

pub async fn job(ip : u32) -> io::Result<()> {
    let one = (ip >> 24) & 255;
    let two = (ip >> 16 ) & 255;
    let tree = (ip >> 8) & 255;
    let four = ip & 255;
    let mut ip = String::with_capacity(26);
    ip.push_str(one.to_string().as_str());
    ip.push('.');
    ip.push_str(two.to_string().as_str());
    ip.push('.');
    ip.push_str(tree.to_string().as_str());
    ip.push('.');
    ip.push_str(four.to_string().as_str());
    ip.push_str(":8080");
    //println!("{} {} {} {}", one, two, tree, four);
    let addr = ip.parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    let stream = socket.connect(addr).await?;

    Ok(())
}