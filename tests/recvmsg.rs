extern crate socket_msg;

use std::time::Duration;
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv6Addr, Ipv4Addr};
use std::io;

use socket_msg::{MsgSocket, BufferContent};

#[test]
fn recvmsg_base_v4() {
    let receiver_sock = UdpSocket::bind("127.0.0.1:12301").unwrap();
    let sender_sock = UdpSocket::bind("127.0.0.1:12302").unwrap();
    let orig = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12302);
    let dest = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12301);

    let mut in_buf = [0u8; 1024];
    let out_buf = [1u8, 2, 3, 4, 5, 6];

    sender_sock.send_to(&out_buf, dest).unwrap();

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::FullMessage(6));
    assert_eq!(addr, orig);
    for i in 0..6 {
        assert_eq!(in_buf[i], out_buf[i]);
    }
}

#[test]
fn recvmsg_base_v6() {
    let receiver_sock = UdpSocket::bind("[::1]:12311").unwrap();
    let sender_sock = UdpSocket::bind("[::1]:12312").unwrap();
    let orig = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12312);
    let dest = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12311);

    let mut in_buf = [0u8; 1024];
    let out_buf = [1u8, 2, 3, 4, 5, 6];

    sender_sock.send_to(&out_buf, dest).unwrap();

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::FullMessage(6));
    assert_eq!(addr, orig);
    for i in 0..6 {
        assert_eq!(in_buf[i], out_buf[i]);
    }
}

#[test]
fn recvmsg_no_msg() {
    let receiver_sock = UdpSocket::bind("[::1]:12320").unwrap();

    let mut in_buf = [0u8; 1024];

    receiver_sock.set_read_timeout(Some(Duration::new(0, 100))).unwrap();
    let err = receiver_sock.recv_from(&mut in_buf).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::WouldBlock);
}

#[test]
fn recvmsg_buf_too_small() {
    let receiver_sock = UdpSocket::bind("[::1]:12331").unwrap();
    let sender_sock = UdpSocket::bind("[::1]:12332").unwrap();
    let orig = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12332);
    let dest = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 12331);

    let mut in_buf = [0u8; 4];
    let out_buf = [1u8, 2, 3, 4, 5, 6];

    sender_sock.send_to(&out_buf, dest).unwrap();

    let (size, addr) = receiver_sock.recvmsg(&mut in_buf).unwrap();
    assert_eq!(size, BufferContent::Partial);
    assert_eq!(addr, orig);
    for i in 0..4 {
        assert_eq!(in_buf[i], out_buf[i]);
    }

    receiver_sock.set_read_timeout(Some(Duration::new(0, 100))).unwrap();
    let err = receiver_sock.recv_from(&mut in_buf).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::WouldBlock);
}
