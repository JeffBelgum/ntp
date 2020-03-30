/*!
# Example
Shows how to use the ntp library to fetch the current time according
to the requested ntp server.

```rust
extern crate chrono;
extern crate ntp;

use chrono::TimeZone;

fn main() {
    let address = "0.pool.ntp.org:123";
    let response = ntp::request(address).unwrap();
    let unix_time = ntp::unix_time::Instant::from(response.transmit_timestamp);
    let local_time = chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _);
    println!("{}", local_time);
}
```
*/

#![recursion_limit = "1024"]

#[macro_use]
extern crate custom_derive;
extern crate conv;
#[macro_use]
extern crate log;
extern crate byteorder;

use protocol::{ReadBytes, ConstPackedSizeBytes, WriteBytes};
use std::io;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub mod protocol;
pub mod unix_time;

/// Send a blocking request to an ntp server with a hardcoded 5 second timeout.
///
///   `addr` can be any valid socket address
///   returns an error if the server cannot be reached or the response is invalid.
///
///   **TODO**: remove hardcoded timeout
pub fn request<A: ToSocketAddrs>(addr: A) -> io::Result<protocol::Packet> {
    // Create a packet for requesting from an NTP server as a client.
    let mut packet = {
        let leap_indicator = protocol::LeapIndicator::default();
        let version = protocol::Version::V4;
        let mode = protocol::Mode::Client;
        let poll = 0;
        let precision = 0;
        let root_delay = protocol::ShortFormat::default();
        let root_dispersion = protocol::ShortFormat::default();
        let transmit_timestamp = unix_time::Instant::now().into();
        let stratum = protocol::Stratum::UNSPECIFIED;
        let src = protocol::PrimarySource::Null;
        let reference_id = protocol::ReferenceIdentifier::PrimarySource(src);
        let reference_timestamp = protocol::TimestampFormat::default();
        let receive_timestamp = protocol::TimestampFormat::default();
        let origin_timestamp = protocol::TimestampFormat::default();
        protocol::Packet {
            leap_indicator,
            version,
            mode,
            stratum,
            poll,
            precision,
            root_delay,
            root_dispersion,
            reference_id,
            reference_timestamp,
            origin_timestamp,
            receive_timestamp,
            transmit_timestamp,
        }
    };

    // Write the packet to a slice of bytes.
    let mut bytes = [0u8; protocol::Packet::PACKED_SIZE_BYTES];
    (&mut bytes[..]).write_bytes(&packet)?;

    // Create the socket from which we will send the packet.
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.set_read_timeout(Some(Duration::from_secs(5)))?;
    sock.set_write_timeout(Some(Duration::from_secs(5)))?;

    // Send the data.
    let sz = sock.send_to(&bytes, addr)?;
    debug!("{:?}", sock.local_addr());
    debug!("sent: {}", sz);

    // Receive the response.
    let res = sock.recv(&mut bytes[..])?;
    debug!("recv: {:?}", res);
    debug!("{:?}", &bytes[..]);

    // Read the received packet from the response.
    packet = (&bytes[..]).read_bytes()?;
    Ok(packet)
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
