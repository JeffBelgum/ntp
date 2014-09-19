extern crate time;

use std::fmt;

#[deriving(Show,PartialEq)]
pub struct ShortFormat {
    pub sec: u16,
    pub frac: u16,
}

impl ShortFormat {
    #[inline]
    pub fn default() -> ShortFormat { ShortFormat { sec: 0, frac: 0 } }
}

#[deriving(PartialEq)]
pub struct TimestampFormat {
    pub sec: u32,
    pub frac: u32,
}

impl TimestampFormat {
    #[inline]
    pub fn default() -> TimestampFormat { TimestampFormat { sec: 0, frac: 0 } }
}

impl fmt::Show for TimestampFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = time::at(FromNtpTime::from_timestamp(*self)).asctime();
        write!(f, "{}", t)
    }
}

pub static EPOCH_DELTA: i64 = 2208988800i64;
static NTP_SCALE: f64 = 4294967295.0_f64;

pub trait ToNtpTime {
    fn to_short(&self) -> ShortFormat;
    fn to_timestamp(&self) -> TimestampFormat;
}

impl ToNtpTime for time::Timespec {
    fn to_short(&self) -> ShortFormat  {
        let sec = self.sec + EPOCH_DELTA;
        let frac = self.nsec as f64 * NTP_SCALE / 10e9;
        ShortFormat { sec: sec as u16, frac: frac as u16 }
    }
    fn to_timestamp(&self) -> TimestampFormat {
        let sec = self.sec + EPOCH_DELTA;
        let frac = self.nsec as f64 * NTP_SCALE / 10e9;
        TimestampFormat { sec: sec as u32, frac: frac as u32 }
    }
}

pub trait FromNtpTime {
    fn from_short(t: ShortFormat) -> Self;
    fn from_timestamp(t: TimestampFormat) -> Self;
}

impl FromNtpTime for time::Timespec {
    fn from_short(t: ShortFormat) -> time::Timespec {
        time::Timespec::new(
            t.sec as i64 - EPOCH_DELTA, 
            (t.frac as f64 / NTP_SCALE * 1e9) as i32
            )
    }
    fn from_timestamp(t: TimestampFormat) -> time::Timespec {
        time::Timespec::new(
            t.sec as i64 - EPOCH_DELTA, 
            (t.frac as f64 / NTP_SCALE * 1e9) as i32
            )
    }

}





#[test]
fn timestamp_conversions() {
    // TODO: well this isn't a test yet!
    // let sys_time = time::get_time();
    // println!("no conversion:  {}", sys_time);
    // let (sec, frac) = sys_time.to_timestamp();
    // let spec: time::Timespec = FromNtpTime::from_timestamp(sec, frac);
    // println!("from_timestamp: {}", spec);
    // println!("{}", sys_time.to_timestamp());
    // println!("{}", sys_time.to_short());
    // println!("({}, {})", sec, frac);
}
