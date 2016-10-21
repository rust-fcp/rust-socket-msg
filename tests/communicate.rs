extern crate socket_msg;

use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv6Addr, Ipv4Addr};

use socket_msg::{MsgSocket, BufferContent};

#[test]
fn communicate_base_v4() {
    let receiver_sock = UdpSocket::bind("127.0.0.1:12301").unwrap();
    let sender_sock = UdpSocket::bind("127.0.0.1:12302").unwrap();
    let orig = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12302);
    let dest = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12301);

    let mut in_buf = [0u8; 1024];
    let out_buf = [1u8, 2, 3, 4, 5, 6];

    sender_sock.sendmsg(&out_buf, &dest).unwrap();

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::FullMessage(6));
    assert_eq!(addr, orig);
    for i in 0..6 {
        assert_eq!(in_buf[i], out_buf[i]);
    }
}

#[test]
fn communicate_base_v6() {
    let receiver_sock = UdpSocket::bind("[::1]:12311").unwrap();
    let sender_sock = UdpSocket::bind("[::1]:12312").unwrap();
    let orig = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12312);
    let dest = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12311);

    let mut in_buf = [0u8; 1024];
    let out_buf = [1u8, 2, 3, 4, 5, 6];

    sender_sock.sendmsg(&out_buf, &dest).unwrap();

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::FullMessage(6));
    assert_eq!(addr, orig);
    for i in 0..6 {
        assert_eq!(in_buf[i], out_buf[i]);
    }
}


#[test]
fn communicate_two_messages() {
    let receiver_sock = UdpSocket::bind("[::1]:12321").unwrap();
    let sender_sock = UdpSocket::bind("[::1]:12322").unwrap();
    let orig = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12322);
    let dest = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12321);

    let mut in_buf = [0u8; 1024];
    let out_buf1 = [1u8, 2, 3, 4, 5, 6];
    let out_buf2 = [7u8, 8, 9];

    sender_sock.sendmsg(&out_buf1, &dest).unwrap();
    sender_sock.sendmsg(&out_buf2, &dest).unwrap();

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::FullMessage(6));
    assert_eq!(addr, orig);
    for i in 0..6 {
        assert_eq!(in_buf[i], out_buf1[i]);
    }

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::FullMessage(3));
    assert_eq!(addr, orig);
    for i in 0..3 {
        assert_eq!(in_buf[i], out_buf2[i]);
    }
}
