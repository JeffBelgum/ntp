//! NTP data structure.
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use conv::TryFrom;
use errors::*;
use formats::timestamp::{ShortFormat, TimestampFormat};
use formats::{LeapIndicator, Mode, PrimarySource, ReferenceIdentifier, Stratum, Version};
use unix_time;

/// Represents the ntp packet data structure
///
/// Excludes extension fields, key identifier, and digest.
#[derive(Debug, PartialEq, Default)]
pub struct Packet {
    pub li: LeapIndicator,
    pub vn: Version,
    pub mode: Mode,
    pub stratum: Stratum,
    pub poll: i8, // Is this unsigned?
    pub precision: i8,
    pub delay: ShortFormat,
    pub dispersion: ShortFormat,
    pub ref_id: ReferenceIdentifier,
    pub ref_time: TimestampFormat,
    pub orig_time: TimestampFormat,
    pub recv_time: TimestampFormat,
    pub transmit_time: TimestampFormat,
}

impl Packet {
    /// Instantiate a new ntp v2 packet in client mode with transmit time
    /// set to `time::now()`.
    pub fn new_client() -> Packet {
        let transmit_time = unix_time::Instant::now().into();
        trace!("{:?}", transmit_time);
        Packet {
            mode: Mode::Client,
            vn: Version::Ver2,
            transmit_time,
            ..Default::default()
        }
    }

    /// Parse an ntp `Packet` from a bytes reader.
    ///
    /// If there are too few bytes in the reader or the result is a malformed
    /// ntp packet, an error is returned.
    pub fn try_from<T: ReadBytesExt>(mut rdr: T) -> Result<Packet> {
        let li_vn_mode = rdr.read_u8()?;
        let li = LeapIndicator::try_from(li_vn_mode >> 6)?;
        let vn = Version::try_from((li_vn_mode >> 3) & 0b111)?;
        let mode = Mode::try_from(li_vn_mode & 0b111)?;

        let stratum = Stratum::new(rdr.read_u8()?);

        let poll = rdr.read_i8()?;
        let precision = rdr.read_i8()?;
        let delay = rdr.read_u32::<NetworkEndian>()?.into();
        let dispersion = rdr.read_u32::<NetworkEndian>()?.into();
        let ref_id_raw = rdr.read_u32::<NetworkEndian>()?.into();
        let ref_id = if stratum.primary() {
            let source = PrimarySource::try_from(ref_id_raw)?;
            ReferenceIdentifier::Primary(source)
        } else if stratum.secondary() {
            ReferenceIdentifier::Secondary(ref_id_raw)
        } else {
            return Err(format!("Unsupported stratum: {}", stratum.get_value()).into());
        };
        let ref_time = rdr.read_u64::<NetworkEndian>()?.into();
        let orig_time = rdr.read_u64::<NetworkEndian>()?.into();
        let recv_time = rdr.read_u64::<NetworkEndian>()?.into();
        let transmit_time = rdr.read_u64::<NetworkEndian>()?.into();

        Ok(Packet {
            li: li,
            vn: vn,
            mode: mode,
            stratum: stratum,
            poll: poll,
            precision: precision,
            delay: delay,
            dispersion: dispersion,
            ref_id: ref_id,
            ref_time: ref_time,
            orig_time: orig_time,
            recv_time: recv_time,
            transmit_time: transmit_time,
        })
    }
}

impl From<Packet> for Vec<u8> {
    fn from(p: Packet) -> Vec<u8> {
        let mut buf = Vec::with_capacity(48);
        let mut li_vn_mode = 0;
        li_vn_mode |= (p.li as u8) << 6;
        li_vn_mode |= (p.vn as u8) << 3;
        li_vn_mode |= p.mode as u8;
        buf.push(li_vn_mode);
        buf.push(p.stratum.get_value());
        buf.push(p.poll as u8);
        buf.push(p.precision as u8);
        buf.write_u32::<NetworkEndian>(p.delay.into())
            .expect("can't fail");
        buf.write_u32::<NetworkEndian>(p.dispersion.into())
            .expect("can't fail");
        buf.write_u32::<NetworkEndian>(p.ref_id.into())
            .expect("can't fail");
        buf.write_u64::<NetworkEndian>(p.ref_time.into())
            .expect("can't fail");
        buf.write_u64::<NetworkEndian>(p.orig_time.into())
            .expect("can't fail");
        buf.write_u64::<NetworkEndian>(p.recv_time.into())
            .expect("can't fail");
        buf.write_u64::<NetworkEndian>(p.transmit_time.into())
            .expect("can't fail");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::Packet;
    use formats::timestamp::{ShortFormat, TimestampFormat};
    use formats::{LeapIndicator, Mode, PrimarySource, ReferenceIdentifier, Stratum, Version};
    use std::io::Cursor;

    #[test]
    fn packet_from_bytes() {
        let input = vec![
            20u8, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169,
            46, 99, 215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215,
            188, 128, 113, 46, 35, 158, 108,
        ];
        let expected_output = Packet {
            li: LeapIndicator::NoWarning,
            vn: Version::Ver2,
            mode: Mode::Server,
            stratum: Stratum::new(1),
            poll: 3,
            precision: -16,
            delay: ShortFormat { sec: 0, frac: 0 },
            dispersion: ShortFormat { sec: 0, frac: 24 },
            ref_id: ReferenceIdentifier::Primary(PrimarySource::CDMA),
            ref_time: TimestampFormat {
                sec: 3619455081,
                frac: 3332976227,
            },
            orig_time: TimestampFormat {
                sec: 3619402178,
                frac: 2670688256,
            },
            recv_time: TimestampFormat {
                sec: 3619455089,
                frac: 770500141,
            },
            transmit_time: TimestampFormat {
                sec: 3619455089,
                frac: 774086252,
            },
        };

        let rdr = Cursor::new(input);
        assert_eq!(expected_output, Packet::try_from(rdr).unwrap());
    }

    #[test]
    fn packet_to_bytes() {
        let expected_output = vec![
            20, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169,
            46, 99, 215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215,
            188, 128, 113, 46, 35, 158, 108,
        ];
        let input = Packet {
            li: LeapIndicator::NoWarning,
            vn: Version::Ver2,
            mode: Mode::Server,
            stratum: Stratum::new(1),
            poll: 3,
            precision: -16,
            delay: ShortFormat { sec: 0, frac: 0 },
            dispersion: ShortFormat { sec: 0, frac: 24 },
            ref_id: ReferenceIdentifier::Primary(PrimarySource::CDMA),
            ref_time: TimestampFormat {
                sec: 3619455081,
                frac: 3332976227,
            },
            orig_time: TimestampFormat {
                sec: 3619402178,
                frac: 2670688256,
            },
            recv_time: TimestampFormat {
                sec: 3619455089,
                frac: 770500141,
            },
            transmit_time: TimestampFormat {
                sec: 3619455089,
                frac: 774086252,
            },
        };
        let output: Vec<u8> = input.into();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn packet_conversion_roundtrip() {
        let input = vec![
            20, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169,
            46, 99, 215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215,
            188, 128, 113, 46, 35, 158, 108,
        ];
        let rdr = Cursor::new(&input);
        let output: Vec<u8> = Packet::try_from(rdr).unwrap().into();
        assert_eq!(input.as_slice(), output.as_slice());
    }
}
