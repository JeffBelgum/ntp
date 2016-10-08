extern crate time;

use std::fmt;

pub static EPOCH_DELTA: i64 = 2208988800i64;
static NTP_SCALE: f64 = 4294967295.0_f64;

#[derive(Debug,PartialEq,Default,Copy,Clone)]
pub struct ShortFormat {
    pub sec: u16,
    pub frac: u16,
}

impl fmt::Display for ShortFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = time::at(time::Timespec::from(*self));
        write!(f, "{}", t.asctime())
    }
}

// Conversions
impl From<ShortFormat> for time::Timespec {
    fn from(t: ShortFormat) -> time::Timespec {
        time::Timespec::new(
            t.sec as i64 - EPOCH_DELTA,
            (t.frac as f64 / NTP_SCALE * 1e9) as i32,
        )
    }
}

impl From<time::Timespec> for ShortFormat {
    fn from(t: time::Timespec) -> ShortFormat {
        let sec = t.sec + EPOCH_DELTA;
        let frac = t.nsec as f64 * NTP_SCALE / 1e10;
        ShortFormat { sec: sec as u16, frac: frac as u16 }
    }
}

impl From<ShortFormat> for u32 {
    fn from(t: ShortFormat) -> u32 {
        (t.sec as u32) << 16 | t.frac as u32
    }
}

impl From<u32> for ShortFormat {
    fn from(t: u32) -> ShortFormat {
        ShortFormat {
            sec: (t >> 16) as u16,
            frac: t as u16,
        }
    }
}


#[derive(Debug,PartialEq,Default,Copy,Clone)]
pub struct TimestampFormat {
    pub sec: u32,
    pub frac: u32,
}

impl fmt::Display for TimestampFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = time::at(time::Timespec::from(*self));
        write!(f, "{}", t.asctime())
    }
}

impl From<TimestampFormat> for time::Timespec {
    fn from(t: TimestampFormat) -> time::Timespec {
        time::Timespec::new(
            t.sec as i64 - EPOCH_DELTA,
            (t.frac as f64 / NTP_SCALE * 1e9) as i32,
        )
    }
}

impl From<time::Timespec> for TimestampFormat {
    fn from(t: time::Timespec) -> TimestampFormat {
        let sec = t.sec + EPOCH_DELTA;
        let frac = t.nsec as f64 * NTP_SCALE / 1e10;
        TimestampFormat { sec: sec as u32, frac: frac as u32 }
    }
}

impl From<TimestampFormat> for u64 {
    fn from(t: TimestampFormat) -> u64 {
        (t.sec as u64) << 32 | t.frac as u64
    }
}

impl From<u64> for TimestampFormat {
    fn from(t: u64) -> TimestampFormat {
        TimestampFormat {
            sec: (t >> 32) as u32,
            frac: t as u32,
        }
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
