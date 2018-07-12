//! How to request an NTP packet from an NTP server.

extern crate chrono;
extern crate ntp;

use chrono::TimeZone;

fn main() {
    let address = "0.pool.ntp.org:123";
    let response: ntp::packet::Packet = ntp::request(address).unwrap();
    let unix_time = ntp::unix_time::Instant::from(response.transmit_time);
    let local_time = chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _);
    println!("{}", local_time);
}
