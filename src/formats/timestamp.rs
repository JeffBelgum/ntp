extern crate time;

#[deriving(Show,PartialEq)]
pub struct ShortFormat {
    pub seconds: u16,
    pub fractions: u16,
}

#[deriving(Show,PartialEq)]
pub struct TimestampFormat {
    pub fractions: u32,
    pub seconds: u32,
}


pub static EPOCH_DELTA: i64 = 2208988800i64;
static NTP_SCALE: f64 = 4294967295.0_f64;

pub trait ToNtpTime {
    fn to_short(&self) -> (u16,u16);
    fn to_timestamp(&self) -> (u32,u32);
}

impl ToNtpTime for time::Timespec {
    fn to_short(&self) -> (u16,u16)  {
        let sec = self.sec + EPOCH_DELTA;
        let frac = self.nsec as f64 * NTP_SCALE / 10e9;
        (sec as u16, frac as u16)
    }
    fn to_timestamp(&self) -> (u32,u32) {
        let sec = self.sec + EPOCH_DELTA;
        let frac = self.nsec as f64 * NTP_SCALE / 10e9;
        (sec as u32, frac as u32)
    }
}

pub trait FromNtpTime {
    fn from_short(sec: u16, frac: u16) -> Self;
    fn from_timestamp( sec: u32, frac: u32) -> Self;
}

impl FromNtpTime for time::Timespec {
    fn from_short(sec: u16, frac: u16) -> time::Timespec {
        time::Timespec::new(
            sec as i64 - EPOCH_DELTA, 
            (frac as f64 / NTP_SCALE * 1e9) as i32
            )
    }
    fn from_timestamp(sec: u32, frac: u32) -> time::Timespec {
        time::Timespec::new(
            sec as i64 - EPOCH_DELTA, 
            (frac as f64 / NTP_SCALE * 1e9) as i32
            )
    }

}





#[test]
fn timestamp_conversions() {
    // TODO: well this isn't a test yet!
    let sys_time = time::get_time();
    println!("no conversion:  {}", sys_time);
    let (sec, frac) = sys_time.to_timestamp();
    let spec: time::Timespec = FromNtpTime::from_timestamp(sec, frac);
    println!("from_timestamp: {}", spec);
    println!("{}", sys_time.to_timestamp());
    println!("{}", sys_time.to_short());
    println!("({}, {})", sec, frac);
}
