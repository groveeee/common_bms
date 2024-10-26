use std::net::IpAddr;
use std::str::FromStr;

// 获取本机IP地址
pub fn get_local_ip() -> IpAddr {
    let mut socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();
    socket.local_addr().unwrap().ip()
}
