//! How to request an NTP packet from an NTP server.

extern crate chrono;
extern crate ntp;

use chrono::TimeZone;

fn local_time(timestamp: ntp::protocol::TimestampFormat) -> chrono::DateTime<chrono::Local> {
    let unix_time = ntp::unix_time::Instant::from(timestamp);
    chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _)
}

fn main() {
    let address = "0.pool.ntp.org:123";
    let response: ntp::protocol::Packet = ntp::request(address).unwrap();
    println!("Timestamps in local time:");
    println!("  reference: {}", local_time(response.reference_timestamp));
    println!("  origin:    {}", local_time(response.origin_timestamp));
    println!("  receive:   {}", local_time(response.receive_timestamp));
    println!("  transmit:  {}", local_time(response.transmit_timestamp));
}
