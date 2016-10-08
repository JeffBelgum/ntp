extern crate ntplib;
extern crate time;

fn main() {
    // ntplib::packed_ids();

    // REF_ID is an ip if stratum >= 2
    // let packet = ntplib::request(env::args().next().as_slice());
    // match packet {
    //     Ok(p) => {
    //         println!("{}", p);
    //         println!("{}", time::at(p.tx_time()).asctime());
    //     },
    //     Err(s) => println!("{}", s),
    // };

    
    // Example
    // let ntp_client = ntplib::ntp_client();
    // let response = ntp_client.request('ntp.efi.com');
    // let efi_time = response.tx_time;
}
