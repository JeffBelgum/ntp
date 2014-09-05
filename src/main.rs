extern crate ntplib;

use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::mem;
use ntplib::{Packet, Timestamp};

fn main() {
    let size = mem::size_of::<Packet>();
    println!("{}", size);
    let p = Packet {
        li: 1,
        vn: 2,
        mode: 3,
        strat: 4,
        poll: 5,
        prec: 6,
        delay: 7,
        disp: 8,
        ref_id: 9,
        ref_time: Timestamp::new(10i32, 11i32),
        orig_time: Timestamp::new(12i32, 13i32),
        recv_time: Timestamp::new(14i32, 15i32),
        transmit_time: Timestamp::new(16i32, 17i32),
    };

    let v: &[u8, ..56] = unsafe { &mem::transmute(p) };
    let V = Vec::from_slice(v);

    println!("{}", V);

    let addr = SocketAddr { ip: Ipv4Addr(10, 3, 24, 172), port: 123 };
    //let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 5001 };
    let myaddr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 5000 };
    let mut socket = match UdpSocket::bind(myaddr) {
        Ok(s) => s,
        Err(e) => fail!("Couldn't bind socket: {}", e),
    };

    let mut req_data = [0u8];
    match socket.send_to(req_data, addr) {
        Ok(_) => println!("sent data!"),
        Err(e) => println!("couldn't send datagram: {}", e),
    };
    //let mut buf = [0, ..256];
    // match socket.recv_from(buf) {
    //     Ok((amt, src)) => println!("received {} bytes from {}", amt, src),
    //     Err(e) => println!("couldn't receive a datagram: {}", e)
    // }
    // Example
    // let ntp_client = ntplib::ntp_client();
    // let response = ntp_client.request('ntp.efi.com');
    // let efi_time = response.tx_time;
}
