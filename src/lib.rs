/*!
# Example

```rust
extern crate ntplib;

fn main() {
    let response: ntplib::Packet = ntplib::request('ntp.efi.com');
    let efi_time = response.tx_time;
    println!("{}", efi_time);
}
```
*/

use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};

pub mod packed;

pub struct Timestamp {
    seconds: i32,
    fractions: i32,
}

impl Timestamp {
    pub fn new(sec: i32, frac: i32) -> Timestamp { Timestamp { seconds: sec, fractions: frac } }
}

// !network native
pub struct Packet {
    pub li: u8,
    pub vn: u8,
    pub mode: u8,
    pub strat: i8,
    pub poll: i32,
    pub prec: i32,
    pub delay: i32,
    pub disp: i32,
    pub ref_id: i32,
    pub ref_time: Timestamp,
    pub orig_time: Timestamp,
    pub recv_time: Timestamp,
    pub transmit_time: Timestamp,
}

impl Packet {
    pub fn new(li: u8, vn: u8, mode: u8, strat: i8,
               poll: i32, prec: i32, delay: i32, disp: i32, ref_id: i32,
               ref_time: Timestamp, orig_time: Timestamp,
               recv_time: Timestamp, transmit_time: Timestamp) -> Packet {
        Packet {
            li: li,
            vn: vn,
            mode: mode,
            strat: strat,
            poll: poll,
            prec: prec,
            delay: delay,
            disp: disp,
            ref_id: ref_id,
            ref_time: ref_time,
            orig_time: orig_time,
            recv_time: recv_time,
            transmit_time: transmit_time,
        }
    }
}
// use datetime
// use socket
// use time

enum RefId { }

enum Stratum { }

enum Mode { }

enum Leap { }
// 
// pub struct Packet {
//     leap: Leap,
//     version: ,
//     mode: Mode,
//     stratum: Stratum,
//     poll: uint,
//     precision: uint,
//     root_delay: uint,
//     root_dispersion: uint,
//     ref_id: RefId,
//     ref_timestamp: uint,
//     orig_timestamp: uint,
//     recv_timestamp: uint,
//     tx_timestamp: uint,
// }
// 
// pub impl Packet {
//     fn to_data(&self) -> &[u8] { }
//     fn from_data(buf: &[u8]) -> Option<Self> { }
// 
//     /* NTPStates */
//     fn offset(&self);
//     fn delay(&self);
//     fn tx_time(&self);
//     fn recv_time(&self);
//     fn orig_time(&self);
//     fn ref_time(&self);
//     fn dest_time(&self);
// }
// 
// pub fn request(host, version, port, timeout) { 
// 
// 
// }
// 
// #[test]
// fn it_works() {
// }
