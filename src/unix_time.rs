use std::time;

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
            },
            Err(sys_time_err) => {
                let duration_pre_unix_epoch = sys_time_err.duration();
                let secs = -(duration_pre_unix_epoch.as_secs() as i64);
                let subsec_nanos = -(duration_pre_unix_epoch.subsec_nanos() as i32);
                Instant::new(secs, subsec_nanos)
            },
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
