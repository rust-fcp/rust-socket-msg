extern crate libc;

use std::os::unix::io::AsRawFd;
use std::net::SocketAddr;
use std::io;
use std::net::UdpSocket;
use std::ptr;
use std::mem;

mod utils;
use utils::{sockaddr_to_addr, addr_to_sockaddr};

/// Indicates how a buffer was filled
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum BufferContent {
    /// Normal case: a full message was put in the buffer.
    /// The parameter is the size of the message.
    FullMessage(usize),
    /// The message was too large for the buffer, so the end was discarded
    Partial,
}

pub trait MsgSocket {
    /// Similar to UdpSocket::recv_from, but only read the content of
    /// one datagram.
    fn recvmsg(&self, buf: &mut [u8]) -> io::Result<(BufferContent, SocketAddr)>;
    /// Similar to UdpSocket::send_to, but sends the buffer in a
    /// single datagram.
    fn sendmsg(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize>;
}

const MSG_TRUNC: i32 =    0x0020;
const MSG_ERRQUEUE: i32 = 0x2000;

impl MsgSocket for UdpSocket {
    fn recvmsg(&self, buf: &mut [u8]) -> io::Result<(BufferContent, SocketAddr)> {
        let sockfd = self.as_raw_fd();
        let flags = 0;
        let mut from_addr_storage: libc::sockaddr_storage = unsafe { mem::zeroed() };
        let addr_len = mem::size_of_val(&from_addr_storage) as libc::socklen_t;
        let mut iovec_item = libc::iovec {
            iov_base: buf.as_mut_ptr() as *mut libc::c_void,
            iov_len: buf.len(),
        };
        let mut msghdr = libc::msghdr {
            msg_name: &mut from_addr_storage as *mut libc::sockaddr_storage as *mut libc::c_void,
            msg_namelen: addr_len,
            msg_iov: &mut iovec_item as *mut libc::iovec,
            msg_iovlen: 1, // Only one buffer
            msg_control: ptr::null::<u8>() as *mut libc::c_void,
            msg_controllen: 0,
            msg_flags: 0,
        };
        let size = unsafe { libc::recvmsg(sockfd, &mut msghdr as *mut libc::msghdr, flags) };
        if size == -1 || (msghdr.msg_flags & MSG_ERRQUEUE) != 0 {
            Err(io::Error::new(io::ErrorKind::WouldBlock, "no message in queue"))
        }
        else if (msghdr.msg_flags & MSG_TRUNC) != 0 {
            Ok((BufferContent::Partial, try!(sockaddr_to_addr(&from_addr_storage, addr_len as usize))))
        }
        else {
            Ok((BufferContent::FullMessage(size as usize), try!(sockaddr_to_addr(&from_addr_storage, addr_len as usize))))
        }
    }

    fn sendmsg(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        let sockfd = self.as_raw_fd();
        let flags = 0;
        let (to_addr_storage, addr_len) = addr_to_sockaddr(addr);
        let mut iovec_item = libc::iovec {
            iov_base: buf.as_ptr() as *mut libc::c_void,
            iov_len: buf.len(),
        };
        let msghdr = libc::msghdr {
            msg_name: to_addr_storage as *const libc::sockaddr_storage as *mut libc::c_void,
            msg_namelen: addr_len,
            msg_iov: &mut iovec_item as *mut libc::iovec,
            msg_iovlen: 1, // Only one buffer
            msg_control: ptr::null::<u8>() as *mut libc::c_void,
            msg_controllen: 0,
            msg_flags: 0, // Unused
        };
        let size = unsafe { libc::sendmsg(sockfd, &msghdr as *const libc::msghdr, flags) };
        if size < 0 {
            Err(io::Error::new(io::ErrorKind::Other, "sendmsg returned -1"))
        }
        else {
            Ok(size as usize)
        }
    }
}
