use unix_time;

pub static EPOCH_DELTA: i64 = 2208988800i64;
static NTP_SCALE: f64 = 4294967295.0_f64;

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct ShortFormat {
    pub sec: u16,
    pub frac: u16,
}

impl From<ShortFormat> for unix_time::Instant {
    fn from(t: ShortFormat) -> unix_time::Instant {
        let secs = t.sec as i64 - EPOCH_DELTA;
        let subsec_nanos = (t.frac as f64 / NTP_SCALE * 1e9) as i32;
        unix_time::Instant::new(secs, subsec_nanos)
    }
}

impl From<unix_time::Instant> for ShortFormat {
    fn from(t: unix_time::Instant) -> ShortFormat {
        let sec = t.secs() + EPOCH_DELTA;
        let frac = t.subsec_nanos() as f64 * NTP_SCALE / 1e10;
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

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct TimestampFormat {
    pub sec: u32,
    pub frac: u32,
}

impl From<unix_time::Instant> for TimestampFormat {
    fn from(t: unix_time::Instant) -> TimestampFormat {
        let sec = t.secs() + EPOCH_DELTA;
        let frac = t.subsec_nanos() as f64 * NTP_SCALE / 1e10;
        TimestampFormat { sec: sec as u32, frac: frac as u32 }
    }
}

impl From<TimestampFormat> for unix_time::Instant {
    fn from(t: TimestampFormat) -> unix_time::Instant {
        let secs = t.sec as i64 - EPOCH_DELTA;
        let subsec_nanos = (t.frac as f64 / NTP_SCALE * 1e9) as i32;
        unix_time::Instant::new(secs, subsec_nanos)
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
