extern crate ntplib;

//use ntplib::packet::Packet;

fn main() {
    // ntplib::packed_ids();

    let packet = ntplib::request("10.3.24.172");
    println!("{}", packet);

    
    // Example
    // let ntp_client = ntplib::ntp_client();
    // let response = ntp_client.request('ntp.efi.com');
    // let efi_time = response.tx_time;
}
