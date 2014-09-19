/*!
# Example

```rust
fn main() {
    // let response: ntplib::packet::Packet = ntplib::request('ntp.efi.com');
    // let efi_time = response.tx_time;
    // println!("{}", efi_time);
}
```
*/

#![feature(macro_rules)]
#![feature(tuple_indexing)]

extern crate syntax;
extern crate time;

use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::from_str::FromStr;

use packet::Packet;

use formats::timestamp::{TimestampFormat, ShortFormat, ToNtpTime};
use formats::{NoWarning, Ver4, Client, Stratum, NULL};

pub mod packet;
pub mod formats;


pub fn request(server_ip: &str) -> Result<packet::Packet, &'static str> {


    // _SYSTEM_EPOCH = datetime.date(*time.gmtime(0)[0:3])
    // """system epoch"""
    // _NTP_EPOCH = datetime.date(1900, 1, 1)
    // """NTP epoch"""
    // NTP_DELTA = (_SYSTEM_EPOCH - _NTP_EPOCH).days * 24 * 3600
    // """delta between system and NTP time"""
    //
    // timestamp + NTP_DELTA // sys to ntp




    let mut socket = match UdpSocket::bind( SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: 0 } ) {
        Ok(s) => s,
        Err(_) => return Err("couldn't bind sockedt"),
    };

    let remote_address = SocketAddr { ip: FromStr::from_str(server_ip).unwrap(), port: 123 };


    let request = Packet {
        li: NoWarning, 
        vn: Ver4,
        mode: Client,
        stratum: Stratum::new(0),
        poll: 0,
        precision: 0,
        delay: ShortFormat::default(),
        dispersion: ShortFormat::default(),
        ref_id: NULL,
        ref_time: TimestampFormat::default(), 
        orig_time: TimestampFormat::default(), 
        recv_time: TimestampFormat::default(), 
        transmit_time: time::get_time().to_timestamp(), 
    };


    match socket.send_to(request.to_bytes().as_slice(), remote_address) {
        Err(_) => return Err("couldn't send data"),
        Ok(_) => (),
    };

    let mut buf = [0, ..48];
    match socket.recv_from(buf) {
        Ok((amt, src)) => { println!("received {} bytes from {}", amt, src); println!("{}", buf.as_slice()); Packet::from_bytes(&buf) }
        Err(_) => return Err("couldn't receive datagram"),
    }

}


















fn pack_code(code: &str) -> (String, Option<u32>) {
    if code.len() > 4 {
        (code.to_string(), None)
    } else {
        (code.to_string(), Some(code.chars().fold(0u32, |acc, el| acc << 8 | (el as u32))))
    }
}
// macro_rules! pack(
//     ($($e:expr),*) => ({
//         $(let (c, p) = pack_code($e);)*
//         println!("{}\t\t:{}", p.unwrap(), c);
//         c
//     });
//     ($($e:expr),+,) => (pack!($($e),+))
//     )

pub fn packed_ids() {
//    pack!("GOES","GPS\n", "IRIG");
    println!("\n");
    let (c, p) = pack_code("GOES");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("GPS\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("GAL\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("PPS\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("IRIG");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("WWVB");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("DCF\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("HBG\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("MSF\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("JJY\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("LORC");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("TDF\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("CHU\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("WWV\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("WWVH");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("NIST");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("ACTS");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("USNO");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("PTB\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("PTB\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("LOCL");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("CESM");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("RBDM");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("OMEG");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("DCN\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("TSP\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("DTS\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("ATOM");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("VLF\0");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("OPPS");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("FREE");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("INIT");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("CDMA");
    println!("{}\t\t:{}", p.unwrap(), c);
    let (c, p) = pack_code("\0\0\0\0");
    println!("{}\t\t:{}", p.unwrap(), c);
}


#[test]
fn pack_them_ids() {
    packed_ids();
}
