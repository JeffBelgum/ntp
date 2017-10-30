/*!
# Example

```rust
fn main() {
    let address = "0.pool.ntp.org:123";
    let response: ntp::packet::Packet = ntp::request(address).unwrap();
    let ntp_time = response.transmit_time;
    println!("{}", ntp_time);
}
```
*/

#![cfg_attr(feature="generators", feature(proc_macro, conservative_impl_trait, generators))]


#![recursion_limit = "1024"] 

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate conv;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate byteorder;
extern crate time;

// async
#[cfg(feature="generators")]
extern crate futures_await as futures;
#[cfg(not(feature="generators"))]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::io::{self, BufReader, Cursor};
use std::net::{SocketAddr,ToSocketAddrs,UdpSocket};
use std::time::Duration;

use futures::prelude::*;
use tokio_core::reactor::{Core, Handle, Timeout};
use tokio_core::net as tokio_net;
use tokio_io::AsyncRead;


pub mod errors;
pub mod formats;
pub mod packet;

pub struct RawPacket([u8; 48]);

impl AsRef<[u8]> for RawPacket {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for RawPacket {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

#[cfg(feature="generators")]
#[async]
pub fn async_request(handle: Handle, remote: SocketAddr) -> errors::Result<packet::Packet> {
    let local_addr = "0.0.0.0:0".parse().expect("Cannot create local address");
    let socket = tokio_net::UdpSocket::bind(&local_addr, &handle)?;
    let mut data = [0; 48];
    packet::Packet::new_client().to_writer(&mut data[..])?;
    let (socket, _) = await!(socket.send_dgram(RawPacket(data), remote))?;
    let mut buf = RawPacket([0; 48]);
    let (socket, resp, n, remote_addr) = await!(socket.recv_dgram(buf))?;
    if remote_addr != remote {
        bail!("Data gram received from unexpected address");
    }
    let rdr = Cursor::new(&resp);
    packet::Packet::try_from(rdr)
}

#[cfg(not(feature="generators"))]
pub fn async_request(handle: Handle, remote: SocketAddr) -> Box<Future<Item=packet::Packet, Error=errors::Error>> {
    let local_addr = "0.0.0.0:0".parse().expect("Cannot create local address");
    let socket = match tokio_net::UdpSocket::bind(&local_addr, &handle) {
        Ok(s) => s,
        Err(e) => return Box::new(futures::future::err::<_, _>(e.into())),
    };
    let mut data = [0; 48];
    if let Err(e) = packet::Packet::new_client().to_writer(&mut data[..]) {
        return Box::new(futures::future::err::<_, _>(e));
    }
    Box::new(socket.send_dgram(RawPacket(data), remote)
        .and_then(|(socket, _)| {
            let mut buf = RawPacket([0; 48]);
            socket.recv_dgram(buf)
        })
        .map_err(|e| e.into())
        .and_then(move |(socket, resp, n, remote_addr)| {
            if remote_addr != remote {
                bail!("Data gram received from unexpected address");
            }
            let rdr = Cursor::new(&resp);
            packet::Packet::try_from(rdr)
        }))
}

pub fn request<A: ToSocketAddrs>(addr: A) -> errors::Result<packet::Packet> {
    let mut data = [0; 48];
    packet::Packet::new_client().to_writer(&mut data[..])?;
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.set_read_timeout(Some(Duration::from_secs(5)))?;
    sock.set_write_timeout(Some(Duration::from_secs(5)))?;
    let sz = sock.send_to(&data, addr)?;
    debug!("{:?}", sock.local_addr());
    debug!("sent: {}", sz);
    let mut buf = [0; 48];
    let res = {
        let buf_ref = &mut buf[..];
        sock.recv(buf_ref)?
    };
    debug!("recv: {:?}", res);
    debug!("{:?}", &buf[..]);
    let rdr = Cursor::new(&buf[..]);
    return Ok(packet::Packet::try_from(rdr)?);
}

#[test]
fn test_request_ntp_org() {
    let res = request("0.pool.ntp.org:123");
    let _ = res.expect("Failed to get a ntp packet from ntp.org");
}

#[test]
fn test_request_google() {
    let res = request("time.google.com:123");
    let _ = res.expect("Failed to get a ntp packet from time.google.com");
}

#[test]
fn test_async_request_ntp_org() {
    let mut core = Core::new().expect("Cannot create tokio core");
    let handle = core.handle();
    let remote = "0.pool.ntp.org:123".to_socket_addrs().unwrap().next().unwrap();
    let handler_future = async_request(handle, remote);
    let result = core.run(handler_future);
    let p = result.expect("Failed to get a ntp packet from ntp.org using async request");
}
