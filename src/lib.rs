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

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate conv;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate byteorder;
extern crate time;

use std::io::Cursor;
use std::net::{ToSocketAddrs,UdpSocket};
use std::time::Duration;



pub mod errors;
pub mod formats;
pub mod packet;

pub fn request<A: ToSocketAddrs>(addr: A) -> errors::Result<packet::Packet> {
    let data: Vec<u8> = packet::Packet::new_client().into();
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.set_read_timeout(Some(Duration::from_secs(5)))?;
    sock.set_write_timeout(Some(Duration::from_secs(5)))?;
    let sz = sock.send_to(&data, addr)?;
    debug!("{:?}", sock.local_addr());
    debug!("sent: {}", sz);
    let mut buf = vec![0; 48];
    let res = sock.recv(&mut buf)?;
    debug!("recv: {:?}", res);
    debug!("{:?}", &buf[..]);
    let rdr = Cursor::new(&buf);
    return Ok(packet::Packet::try_from(rdr)?);
}

#[test]
fn test_request() {
    let res = request("0.pool.ntp.org:123");
    let _ = res.expect("Failed to get a ntp packet from ntp.org");
}

