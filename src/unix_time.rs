use crate::protocol;
use std::{self, time};

/// The number of seconds from 1st January 1900 UTC to the start of the Unix epoch.
pub const EPOCH_DELTA: i64 = 2_208_988_800;

// The NTP fractional scale.
const NTP_SCALE: f64 = std::u32::MAX as f64 + 1.0;

/// Describes an instant relative to the `UNIX_EPOCH` - 00:00:00 Coordinated Universal Time (UTC),
/// Thursay, 1 January 1970 in seconds with the fractional part in nanoseconds.
///
/// If the **Instant** describes some moment prior to `UNIX_EPOCH`, both the `secs` and
/// `subsec_nanos` components will be negative.
///
/// The sole purpose of this type is for retrieving the "current" time using the `std::time` module
/// and for converting between the ntp timestamp formats. If you are interested in converting from
/// unix time to some other more human readable format, perhaps see the [chrono
/// crate](https://crates.io/crates/chrono).
///
/// ## Example
///
/// Here is a demonstration of displaying the **Instant** in local time using the chrono crate:
///
/// ```
/// extern crate chrono;
/// extern crate ntp;
///
/// use chrono::TimeZone;
///
/// fn main() {
///     let unix_time = ntp::unix_time::Instant::now();
///     let local_time = chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _);
///     println!("{}", local_time);
/// }
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Instant {
    secs: i64,
    subsec_nanos: i32,
}

impl Instant {
    /// Create a new **Instant** given its `secs` and `subsec_nanos` components.
    ///
    /// To indicate a time following `UNIX_EPOCH`, both `secs` and `subsec_nanos` must be positive.
    /// To indicate a time prior to `UNIX_EPOCH`, both `secs` and `subsec_nanos` must be negative.
    /// Violating these invariants will result in a **panic!**.
    pub fn new(secs: i64, subsec_nanos: i32) -> Instant {
        if secs > 0 && subsec_nanos < 0 {
            panic!("invalid instant: secs was positive but subsec_nanos was negative");
        }
        if secs < 0 && subsec_nanos > 0 {
            panic!("invalid instant: secs was negative but subsec_nanos was positive");
        }
        Instant { secs, subsec_nanos }
    }

    /// Uses `std::time::SystemTime::now` and `std::time::UNIX_EPOCH` to determine the current
    /// **Instant**.
    ///
    /// ## Example
    ///
    /// ```
    /// extern crate ntp;
    ///
    /// fn main() {
    ///     println!("{:?}", ntp::unix_time::Instant::now());
    /// }
    /// ```
    pub fn now() -> Self {
        match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs() as i64;
                let subsec_nanos = duration.subsec_nanos() as i32;
                Instant::new(secs, subsec_nanos)
            }
            Err(sys_time_err) => {
                let duration_pre_unix_epoch = sys_time_err.duration();
                let secs = -(duration_pre_unix_epoch.as_secs() as i64);
                let subsec_nanos = -(duration_pre_unix_epoch.subsec_nanos() as i32);
                Instant::new(secs, subsec_nanos)
            }
        }
    }

    /// The "seconds" component of the **Instant**.
    pub fn secs(&self) -> i64 {
        self.secs
    }

    /// The fractional component of the **Instant** in nanoseconds.
    pub fn subsec_nanos(&self) -> i32 {
        self.subsec_nanos
    }
}

// Conversion implementations.

impl From<protocol::ShortFormat> for Instant {
    fn from(t: protocol::ShortFormat) -> Self {
        let secs = i64::from(t.seconds) - EPOCH_DELTA;
        let subsec_nanos = (f64::from(t.fraction) / NTP_SCALE * 1e9) as i32;
        Instant::new(secs, subsec_nanos)
    }
}

impl From<protocol::TimestampFormat> for Instant {
    fn from(t: protocol::TimestampFormat) -> Self {
        let secs = i64::from(t.seconds) - EPOCH_DELTA;
        let subsec_nanos = (f64::from(t.fraction) / NTP_SCALE * 1e9) as i32;
        Instant::new(secs, subsec_nanos)
    }
}

impl From<Instant> for protocol::ShortFormat {
    fn from(t: Instant) -> Self {
        let sec = t.secs() + EPOCH_DELTA;
        let frac = f64::from(t.subsec_nanos()) * NTP_SCALE / 1e9;
        protocol::ShortFormat {
            seconds: sec as u16,
            fraction: frac as u16,
        }
    }
}

impl From<Instant> for protocol::TimestampFormat {
    fn from(t: Instant) -> Self {
        let sec = t.secs() + EPOCH_DELTA;
        let frac = f64::from(t.subsec_nanos()) * NTP_SCALE / 1e9;
        protocol::TimestampFormat {
            seconds: sec as u32,
            fraction: frac as u32,
        }
    }
}
