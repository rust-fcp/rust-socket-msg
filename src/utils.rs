extern crate libc;
use std::io;
use std::mem;
use std::net::SocketAddr;
use std::net::{SocketAddrV4, SocketAddrV6};

// Taken from https://github.com/rust-lang/rust/blob/14f9cbdfd596390e039a7af8ca3003662fecc28e/src/libstd/sys/common/net.rs#L94
pub fn sockaddr_to_addr(storage: &libc::sockaddr_storage,
                    len: usize) -> io::Result<SocketAddr> {
    match storage.ss_family as libc::c_int {
        libc::AF_INET => {
            assert!(len as usize >= mem::size_of::<libc::sockaddr_in>());
            Ok(SocketAddr::V4(unsafe { *(storage as *const _ as *const SocketAddrV4) }))
        }
        libc::AF_INET6 => {
            assert!(len as usize >= mem::size_of::<libc::sockaddr_in6>());
            Ok(SocketAddr::V6(unsafe { *(storage as *const _ as *const SocketAddrV6) }))
        }
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidInput, format!("invalid argument to sockaddr_to_addr: {:?}", storage.ss_family)))
        }
    }
}

/// 'addr' must live longer than the returned sockaddr_storage.
pub fn addr_to_sockaddr(addr: &SocketAddr) -> (*const libc::sockaddr_storage, libc::socklen_t) {
    match *addr {
        SocketAddr::V4(ref a) => {
            (a as *const _ as *const _, mem::size_of_val(a) as libc::socklen_t)
        }
        SocketAddr::V6(ref a) => {
            (a as *const _ as *const _, mem::size_of_val(a) as libc::socklen_t)
        }
    }
}
