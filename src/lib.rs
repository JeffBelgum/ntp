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

#![recursion_limit = "1024"] 
// TODO remove dependence on nightly
#![feature(try_from)] 

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate conv;
#[macro_use] extern crate error_chain;
extern crate byteorder;
extern crate time;

use std::net::{ToSocketAddrs,UdpSocket};
use std::io::Cursor;
use std::convert::TryFrom;



pub mod errors;
pub mod formats;
pub mod packet;

pub fn request<A: ToSocketAddrs>(addr: A) -> errors::Result<packet::Packet> {
    let data: Vec<u8> = packet::Packet::new_client().into();
    // FIXME TODO don't hardcode a port!
    let sock = UdpSocket::bind("0.0.0.0:5679")?;
    // FIXME TODO set some sensible socket r/w timeouts
    let sz = sock.send_to(&data, addr)?;
    println!("{:?}", sock.local_addr());
    println!("sent: {}", sz);
    let mut buf = vec![0; 48];
    let res = sock.recv(&mut buf)?;
    println!("recv: {:?}", res);
    println!("{:?}", &buf[..]);
    let rdr = Cursor::new(&buf);
    return Ok(packet::Packet::try_from(rdr)?);
}

#[test]
fn test_request() {
    let res = request("0.pool.ntp.org:123");
    let p = res.expect("Failed to get a ntp packet from ntp.org");
}

