//! How to request an NTP packet from an NTP server.

extern crate ntp;

fn main() {
    let address = "0.pool.ntp.org:123";
    let response: ntp::packet::Packet = ntp::request(address).unwrap();
    let ntp_time = response.transmit_time;
    println!("{}", ntp_time);
}
