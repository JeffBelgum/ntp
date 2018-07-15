//! Types and constants that precisely match the specification.
//!
//! Provides `ReadBytes` and `WriteBytes` implementations which extend the byteorder crate
//! `WriteBytesExt` and `ReadBytesExt` traits with the ability to read and write types from the NTP
//! protocol respectively.
//!
//! Documentation is largely derived (and often copied directly) from IETF RFC 5905.

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use conv::TryFrom;
use std::{fmt, io};

/// NTP port number.
pub const PORT: u8 = 123;

/// Frequency tolerance PHI (s/s).
pub const TOLERANCE: f64 = 15e-6;

/// Minimum poll exponent (16 s).
pub const MINPOLL: u8 = 4;

/// Maximum poll exponent (36 h).
pub const MAXPOLL: u8 = 17;

/// Maximum dispersion (16 s).
pub const MAXDISP: f64 = 16.0;

/// Minimum dispersion increment (s).
pub const MINDISP: f64 = 0.005;

/// Distance threshold (1 s).
pub const MAXDIST: u8 = 1;

/// Maximum stratum number.
pub const MAXSTRAT: u8 = 16;

/// A trait for writing any of the Network Time Protocol types to network-endian bytes.
///
/// A blanket implementation is provided for all types that implement `byteorder::WriteBytesExt`.
pub trait WriteBytes {
    fn write_bytes<P: WriteToBytes>(&mut self, protocol: P) -> io::Result<()>;
}

/// A trait for reading any of the Network Time Protocol types from network-endian bytes.
///
/// A blanket implementation is provided for all types that implement `byteorder::ReadBytesExt`.
pub trait ReadBytes {
    fn read_bytes<P: ReadFromBytes>(&mut self) -> io::Result<P>;
}

/// Network Time Protocol types that may be written to network endian bytes.
pub trait WriteToBytes {
    /// Write the command to bytes.
    fn write_to_bytes<W: WriteBytesExt>(&self, W) -> io::Result<()>;
}

/// Network Time Protocol types that may be read from network endian bytes.
pub trait ReadFromBytes: Sized {
    /// Read the command from bytes.
    fn read_from_bytes<R: ReadBytesExt>(R) -> io::Result<Self>;
}

/// Types that have a constant size when written to or read from bytes.
pub trait ConstPackedSizeBytes {
    const PACKED_SIZE_BYTES: usize;
}

/// **NTP Short Format** - Used in delay and dispersion header fields where the full resolution and
/// range of the other formats are not justified. It includes a 16-bit unsigned seconds field and a
/// 16-bit fraction field.
///
/// ### Layout
///
/// ```ignore
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          Seconds              |           Fraction            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ShortFormat {
    pub seconds: u16,
    pub fraction: u16,
}

/// **NTP Timestamp Format** - Used in packet headers and other places with limited word size. It
/// includes a 32-bit unsigned seconds field spanning 136 years and a 32-bit fraction field
/// resolving 232 picoseconds.
///
/// The prime epoch is 0 h 1 January 1900 UTC, when all bits are zero.
///
/// ### Layout
///
/// ```ignore
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                            Seconds                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                            Fraction                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimestampFormat {
    pub seconds: u32,
    pub fraction: u32,
}

/// **NTP Date Format** - The prime epoch, or base date of era 0, is 0 h 1 January 1900 UTC, when all
/// bits are zero. Dates are relative to the prime epoch; values greater than zero represent times
/// after that date; values less than zero represent times before it.
///
/// Note that the `era_offset` field has the same interpretation as the `seconds` field of the
/// `TimestampFormat` type.
///
/// ```ignore
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           Era Number                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           Era Offset                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                           Fraction                            |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateFormat {
    pub era_number: i32,
    pub era_offset: u32,
    pub fraction: u64,
}

custom_derive! {
    /// A 2-bit integer warning of an impending leap second to be inserted or deleted in the last
    /// minute of the current month with values defined below:
    ///
    /// Note that this field is packed in the actual header.
    ///
    /// As the only constructors are via associated constants, it should be impossible to create an
    /// invalid `LeapIndicator`.
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, TryFrom(u8))]
    pub enum LeapIndicator {
        /// No leap required.
        NoWarning = 0,
        /// Last minute of the day has 61 seconds.
        AddOne = 1,
        /// Last minute of the day has 59 seconds.
        SubOne = 2,
        /// Clock unsynchronized.
        Unknown = 3,
    }
}

/// A 3-bit integer representing the NTP version number, currently 4.
///
/// Note that while this struct is 8-bits, this field is packed to 3 in the actual header.
///
/// As the only constructors are via associated constants, it should be impossible to create an
/// invalid `Version`.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version(u8);

custom_derive! {
    /// A 3-bit integer representing the mode.
    ///
    /// Note that while this struct is 8-bits, this field is packed to 3 in the actual header.
    ///
    /// As the only constructors are via associated constants, it should be impossible to create an
    /// invalid `Mode`.
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, TryFrom(u8))]
    pub enum Mode {
        Reserved = 0,
        SymmetricActive = 1,
        SymmetricPassive = 2,
        Client = 3,
        Server = 4,
        Broadcast = 5,
        NtpControlMessage = 6,
        ReservedForPrivateUse = 7,
    }
}

/// An 8-bit integer representing the stratum.
///
/// ```ignore
/// +--------+-----------------------------------------------------+
/// | Value  | Meaning                                             |
/// +--------+-----------------------------------------------------+
/// | 0      | unspecified or invalid                              |
/// | 1      | primary server (e.g., equipped with a GPS receiver) |
/// | 2-15   | secondary server (via NTP)                          |
/// | 16     | unsynchronized                                      |
/// | 17-255 | reserved                                            |
/// +--------+-----------------------------------------------------+
/// ```
///
/// It is customary to map the stratum value 0 in received packets to `MAXSTRAT` in the peer
/// variable p.stratum and to map p.stratum values of `MAXSTRAT` or greater to 0 in transmitted
/// packets. This allows reference clocks, which normally appear at stratum 0, to be conveniently
/// mitigated using the same clock selection algorithms used for external sources.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Stratum(pub u8);

/// A 32-bit code identifying the particular server or reference clock.
///
/// The interpretation depends on the value in the stratum field:
///
/// - For packet stratum 0 (unspecified or invalid), this is a four-character ASCII [RFC1345]
///   string, called the "kiss code", used for debugging and monitoring purposes.
/// - For stratum 1 (reference clock), this is a four-octet, left-justified, zero-padded ASCII
///   string assigned to the reference clock.
///
/// The authoritative list of Reference Identifiers is maintained by IANA; however, any string
/// beginning with the ASCII character "X" is reserved for unregistered experimentation and
/// development.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReferenceIdentifier {
    PrimarySource(PrimarySource),
    /// The reference identifier of the secondary or client server. Can be used to detect timing
    /// loops.
    ///
    /// If using the IPv4 address family, the identifier is the four-octet IPv4 address.
    ///
    /// If using the IPv6 address family, it is the first four octets of the MD5 hash of the IPv6
    /// address. Note that when using the IPv6 address family on a NTPv4 server with a NTPv3
    /// client, the Reference Identifier field appears to be a random value and a timing loop might
    /// not be detected.
    SecondaryOrClient([u8; 4]),
    KissOfDeath(KissOfDeath),
}

// Convert an ascii string to a big-endian u32.
macro_rules! code_to_u32 {
    ($w:expr) => {
        (($w[3] as u32) << 0) |
        (($w[2] as u32) << 8) |
        (($w[1] as u32) << 16) |
        (($w[0] as u32) << 24) |
        ((*$w as [u8; 4])[0] as u32 * 0)
    };
}

custom_derive! {
    /// A four-octet, left-justified, zero-padded ASCII string assigned to the reference clock.
    ///
    /// The authoritative list of Reference Identifiers is maintained by IANA; however, any string
    /// beginning with the ASCII character "X" is reserved for unregistered experimentation and
    /// development.
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, TryFrom(u32))]
    pub enum PrimarySource {
        Goes = code_to_u32!(b"GOES"),
        Gps = code_to_u32!(b"GPS\0"),
        Cdma = code_to_u32!(b"CDMA"),
        Gal = code_to_u32!(b"GAL\0"),
        Pps = code_to_u32!(b"PPS\0"),
        Irig = code_to_u32!(b"IRIG"),
        Wwvb = code_to_u32!(b"WWVB"),
        Dcf = code_to_u32!(b"DCF\0"),
        Hgb = code_to_u32!(b"HGB\0"),
        Msf = code_to_u32!(b"MSF\0"),
        Jjy = code_to_u32!(b"JJY\0"),
        Lorc = code_to_u32!(b"LORC"),
        Tdf = code_to_u32!(b"TDF\0"),
        Chu = code_to_u32!(b"CHU\0"),
        Wwv = code_to_u32!(b"WWV\0"),
        Wwvh = code_to_u32!(b"WWVH"),
        Nist = code_to_u32!(b"NIST"),
        Acts = code_to_u32!(b"ACTS"),
        Usno = code_to_u32!(b"USNO"),
        Ptb = code_to_u32!(b"PTB\0"),
        Goog = code_to_u32!(b"GOOG"),
        Locl = code_to_u32!(b"LOCL"),
        Cesm = code_to_u32!(b"CESM"),
        Rbdm = code_to_u32!(b"RBDM"),
        Omeg = code_to_u32!(b"OMEG"),
        Dcn = code_to_u32!(b"DCN\0"),
        Tsp = code_to_u32!(b"TSP\0"),
        Dts = code_to_u32!(b"DTS\0"),
        Atom = code_to_u32!(b"ATOM"),
        Vlf = code_to_u32!(b"VLF\0"),
        Opps = code_to_u32!(b"OPPS"),
        Free = code_to_u32!(b"FREE"),
        Init = code_to_u32!(b"INIT"),
        Null = 0,
    }
}

custom_derive! {
    /// If the Stratum field is 0, which implies unspecified or invalid, the Reference Identifier
    /// field can be used to convey messages useful for status reporting and access control. These
    /// are called **Kiss-o'-Death** (KoD) packets and the ASCII messages they convey are called
    /// kiss codes.
    ///
    /// The KoD packets got their name because an early use was to tell clients to stop sending
    /// packets that violate server access controls. The kiss codes can provide useful information
    /// for an intelligent client, either NTPv4 or SNTPv4. Kiss codes are encoded in four-character
    /// ASCII strings that are left justified and zero filled. The strings are designed for
    /// character displays and log files.
    /// 
    /// Recipients of kiss codes MUST inspect them and, in the following cases, take the actions
    /// described.
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, TryFrom(u32))]
    pub enum KissOfDeath {
        /// The client MUST demobilize any associations to that server and stop sending packets to it.
        Deny = code_to_u32!(b"DENY"),
        /// The client MUST demobilize any associations to that server and stop sending packets to it.
        Rstr = code_to_u32!(b"RSTR"),
        /// The client MUST immediately reduce its polling interval to that server and continue to
        /// reduce it each time it receives a RATE kiss code.
        Rate = code_to_u32!(b"RATE"),
    }
}

/// **Packet Header** - The most important state variables from an external point of view are the
/// packet header variables described here.
///
/// The NTP packet header consists of an integral number of 32-bit (4 octet) words in network byte
/// order. The packet format consists of three components: the header itself, one or more optional
/// extension fields, and an optional message authentication code (MAC).
///
/// ```ignore
/// +-----------+------------+-----------------------+
/// | Name      | Formula    | Description           |
/// +-----------+------------+-----------------------+
/// | leap      | leap       | leap indicator (LI)   |
/// | version   | version    | version number (VN)   |
/// | mode      | mode       | mode                  |
/// | stratum   | stratum    | stratum               |
/// | poll      | poll       | poll exponent         |
/// | precision | rho        | precision exponent    |
/// | rootdelay | delta_r    | root delay            |
/// | rootdisp  | epsilon_r  | root dispersion       |
/// | refid     | refid      | reference ID          |
/// | reftime   | reftime    | reference timestamp   |
/// | org       | T1         | origin timestamp      |
/// | rec       | T2         | receive timestamp     |
/// | xmt       | T3         | transmit timestamp    |
/// | dst       | T4         | destination timestamp |
/// | keyid     | keyid      | key ID                |
/// | dgst      | dgst       | message digest        |
/// +-----------+------------+-----------------------+
/// ```
///
/// ### Format
///
/// The NTP packet is a UDP datagram [RFC0768]. Some fields use multiple words and others are
/// packed in smaller fields within a word. The NTP packet header shown below has 12 words followed
/// by optional extension fields and finally an optional message authentication code (MAC)
/// consisting of the Key Identifier field and Message Digest field.
///
/// ```ignore
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |LI | VN  |Mode |    Stratum     |     Poll      |  Precision   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Root Delay                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Root Dispersion                       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          Reference ID                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                     Reference Timestamp (64)                  +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                      Origin Timestamp (64)                    +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                      Receive Timestamp (64)                   +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                      Transmit Timestamp (64)                  +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// .                                                               .
/// .                    Extension Field 1 (variable)               .
/// .                                                               .
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// .                                                               .
/// .                    Extension Field 2 (variable)               .
/// .                                                               .
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          Key Identifier                       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                            dgst (128)                         |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Packet {
    pub leap_indicator: LeapIndicator,
    pub version: Version,
    pub mode: Mode,
    pub stratum: Stratum,
    /// 8-bit signed integer representing the maximum interval between successive messages, in log2
    /// seconds. Suggested default limits for minimum and maximum poll intervals are 6 and 10,
    /// respectively.
    pub poll: i8,
    /// 8-bit signed integer representing the precision of the system clock, in log2 seconds. For
    /// instance, a value of -18 corresponds to a precision of about one microsecond. The precision
    /// can be determined when the service first starts up as the minimum time of several
    /// iterations to read the system clock.
    pub precision: i8,
    /// Total round-trip delay to the reference clock, in NTP short format.
    pub root_delay: ShortFormat,
    /// Total dispersion to the reference clock, in NTP short format.
    pub root_dispersion: ShortFormat,
    pub reference_id: ReferenceIdentifier,
    /// Time when the system clock was last set or corrected.
    pub reference_timestamp: TimestampFormat,
    /// Time at the client when the request departed for the server.
    pub origin_timestamp: TimestampFormat,
    /// Time at the server when the request arrived from the client.
    pub receive_timestamp: TimestampFormat,
    /// Time at the server when the response left for the client.
    pub transmit_timestamp: TimestampFormat,
}

/// The consecutive types within the first packed byte in the NTP packet.
pub type PacketByte1 = (LeapIndicator, Version, Mode);

// Inherent implementations.

impl PrimarySource {
    /// The bytestring representation of the primary source.
    pub fn bytes(&self) -> [u8; 4] {
        be_u32_to_bytes(*self as u32)
    }
}

impl Version {
    pub const V1: Self = Version(1);
    pub const V2: Self = Version(2);
    pub const V3: Self = Version(3);
    pub const V4: Self = Version(4);

    /// Whether or not the version is a known, valid version.
    pub fn is_known(&self) -> bool {
        self.0 >= 1 && self.0 <= 4
    }
}

impl Stratum {
    /// Unspecified or invalid.
    pub const UNSPECIFIED: Self = Stratum(0);
    /// The primary server (e.g. equipped with a GPS receiver.
    pub const PRIMARY: Self = Stratum(1);
    /// The minimum value specifying a secondary server (via NTP).
    pub const SECONDARY_MIN: Self = Stratum(2);
    /// The maximum value specifying a secondary server (via NTP).
    pub const SECONDARY_MAX: Self = Stratum(15);
    /// An unsynchronized stratum.
    pub const UNSYNCHRONIZED: Self = Stratum(16);
    /// The maximum valid stratum value.
    pub const MAX: Self = Stratum(16);

    /// Whether or not the stratum represents a secondary server.
    pub fn is_secondary(&self) -> bool {
        Self::SECONDARY_MIN <= *self && *self <= Self::SECONDARY_MAX
    }

    /// Whether or not the stratum is in the reserved range.
    pub fn is_reserved(&self) -> bool {
        *self > Self::MAX
    }
}

// Size implementations.

impl ConstPackedSizeBytes for ShortFormat {
    const PACKED_SIZE_BYTES: usize = 4;
}

impl ConstPackedSizeBytes for TimestampFormat {
    const PACKED_SIZE_BYTES: usize = 8;
}

impl ConstPackedSizeBytes for DateFormat {
    const PACKED_SIZE_BYTES: usize = 16;
}

impl ConstPackedSizeBytes for Stratum {
    const PACKED_SIZE_BYTES: usize = 1;
}

impl ConstPackedSizeBytes for ReferenceIdentifier {
    const PACKED_SIZE_BYTES: usize = 4;
}

impl ConstPackedSizeBytes for PacketByte1 {
    const PACKED_SIZE_BYTES: usize = 1;
}

impl ConstPackedSizeBytes for Packet {
    const PACKED_SIZE_BYTES: usize =
        PacketByte1::PACKED_SIZE_BYTES
        + Stratum::PACKED_SIZE_BYTES
        + 2
        + ShortFormat::PACKED_SIZE_BYTES * 2
        + ReferenceIdentifier::PACKED_SIZE_BYTES
        + TimestampFormat::PACKED_SIZE_BYTES * 4;
}

// Writer implementations.

impl<W> WriteBytes for W
where
    W: WriteBytesExt,
{
    fn write_bytes<P: WriteToBytes>(&mut self, protocol: P) -> io::Result<()> {
        protocol.write_to_bytes(self)
    }
}

impl<'a, P> WriteToBytes for &'a P
where
    P: WriteToBytes,
{
    fn write_to_bytes<W: WriteBytesExt>(&self, writer: W) -> io::Result<()> {
        (*self).write_to_bytes(writer)
    }
}

impl WriteToBytes for ShortFormat {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u16::<BE>(self.seconds)?;
        writer.write_u16::<BE>(self.fraction)?;
        Ok(())
    }
}

impl WriteToBytes for TimestampFormat {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u32::<BE>(self.seconds)?;
        writer.write_u32::<BE>(self.fraction)?;
        Ok(())
    }
}

impl WriteToBytes for DateFormat {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        writer.write_i32::<BE>(self.era_number)?;
        writer.write_u32::<BE>(self.era_offset)?;
        writer.write_u64::<BE>(self.fraction)?;
        Ok(())
    }
}

impl WriteToBytes for Stratum {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u8(self.0)?;
        Ok(())
    }
}

impl WriteToBytes for ReferenceIdentifier {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        match *self {
            ReferenceIdentifier::KissOfDeath(kod) => {
                writer.write_u32::<BE>(kod as u32)?;
            }
            ReferenceIdentifier::PrimarySource(src) => {
                writer.write_u32::<BE>(src as u32)?;
            }
            ReferenceIdentifier::SecondaryOrClient(arr) => {
                writer.write_u32::<BE>(code_to_u32!(&arr))?;
            }
        }
        Ok(())
    }
}

impl WriteToBytes for (LeapIndicator, Version, Mode) {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        let (li, vn, mode) = *self;
        let mut li_vn_mode = 0;
        li_vn_mode |= (li as u8) << 6;
        li_vn_mode |= vn.0 << 3;
        li_vn_mode |= mode as u8;
        writer.write_u8(li_vn_mode)?;
        Ok(())
    }
}

impl WriteToBytes for Packet {
    fn write_to_bytes<W: WriteBytesExt>(&self, mut writer: W) -> io::Result<()> {
        let li_vn_mode = (self.leap_indicator, self.version, self.mode);
        writer.write_bytes(li_vn_mode)?;
        writer.write_bytes(self.stratum)?;
        writer.write_i8(self.poll)?;
        writer.write_i8(self.precision)?;
        writer.write_bytes(self.root_delay)?;
        writer.write_bytes(self.root_dispersion)?;
        writer.write_bytes(self.reference_id)?;
        writer.write_bytes(self.reference_timestamp)?;
        writer.write_bytes(self.origin_timestamp)?;
        writer.write_bytes(self.receive_timestamp)?;
        writer.write_bytes(self.transmit_timestamp)?;
        Ok(())
    }
}

// Reader implementations.

impl<R> ReadBytes for R
where
    R: ReadBytesExt,
{
    fn read_bytes<P: ReadFromBytes>(&mut self) -> io::Result<P> {
        P::read_from_bytes(self)
    }
}

impl ReadFromBytes for ShortFormat {
    fn read_from_bytes<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let seconds = reader.read_u16::<BE>()?;
        let fraction = reader.read_u16::<BE>()?;
        let short_format = ShortFormat { seconds, fraction };
        Ok(short_format)
    }
}

impl ReadFromBytes for TimestampFormat {
    fn read_from_bytes<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let seconds = reader.read_u32::<BE>()?;
        let fraction = reader.read_u32::<BE>()?;
        let timestamp_format = TimestampFormat { seconds, fraction };
        Ok(timestamp_format)
    }
}

impl ReadFromBytes for DateFormat {
    fn read_from_bytes<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let era_number = reader.read_i32::<BE>()?;
        let era_offset = reader.read_u32::<BE>()?;
        let fraction = reader.read_u64::<BE>()?;
        let date_format = DateFormat { era_number, era_offset, fraction };
        Ok(date_format)
    }
}

impl ReadFromBytes for Stratum {
    fn read_from_bytes<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let stratum = Stratum(reader.read_u8()?);
        Ok(stratum)
    }
}

impl ReadFromBytes for (LeapIndicator, Version, Mode) {
    fn read_from_bytes<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let li_vn_mode = reader.read_u8()?;
        let li_u8 = li_vn_mode >> 6;
        let vn_u8 = (li_vn_mode >> 3) & 0b111;
        let mode_u8 = li_vn_mode & 0b111;
        let li = match LeapIndicator::try_from(li_u8).ok() {
            Some(li) => li,
            None => {
                let err_msg = "unknown leap indicator";
                return Err(io::Error::new(io::ErrorKind::InvalidData, err_msg));
            },
        };
        let vn = Version(vn_u8);
        let mode = match Mode::try_from(mode_u8).ok() {
            Some(mode) => mode,
            None => {
                let err_msg = "unknown association mode";
                return Err(io::Error::new(io::ErrorKind::InvalidData, err_msg));
            },
        };
        Ok((li, vn, mode))
    }
}

impl ReadFromBytes for Packet {
    fn read_from_bytes<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let (leap_indicator, version, mode) = reader.read_bytes()?;
        let stratum = reader.read_bytes::<Stratum>()?;
        let poll = reader.read_i8()?;
        let precision = reader.read_i8()?;
        let root_delay = reader.read_bytes()?;
        let root_dispersion = reader.read_bytes()?;
        let reference_id = {
            let u = reader.read_u32::<BE>()?;
            if stratum == Stratum::PRIMARY {
                match PrimarySource::try_from(u) {
                    Ok(src) => ReferenceIdentifier::PrimarySource(src),
                    Err(_) => match KissOfDeath::try_from(u) {
                        Ok(kod) => ReferenceIdentifier::KissOfDeath(kod),
                        Err(_) => {
                            let err_msg = "unknown reference id";
                            return Err(io::Error::new(io::ErrorKind::InvalidData, err_msg));
                        }
                    },
                }
            } else if stratum.is_secondary() {
                let arr = be_u32_to_bytes(u);
                ReferenceIdentifier::SecondaryOrClient(arr)
            } else {
                let err_msg = "unsupported stratum";
                return Err(io::Error::new(io::ErrorKind::InvalidData, err_msg));
            }
        };
        let reference_timestamp = reader.read_bytes()?;
        let origin_timestamp = reader.read_bytes()?;
        let receive_timestamp = reader.read_bytes()?;
        let transmit_timestamp = reader.read_bytes()?;
        Ok(Packet {
            leap_indicator,
            version,
            mode,
            stratum,
            poll,
            precision,
            root_delay,
            root_dispersion,
            reference_id,
            reference_timestamp,
            origin_timestamp,
            receive_timestamp,
            transmit_timestamp,
        })
    }
}

// Manual default implementations.

impl Default for LeapIndicator {
    fn default() -> Self {
        LeapIndicator::NoWarning
    }
}

// Display implementations.

impl fmt::Display for PrimarySource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.bytes();
        let s = String::from_utf8_lossy(&bytes);
        write!(f, "{}", s)
    }
}

// Utility functions.

fn be_u32_to_bytes(u: u32) -> [u8; 4] {
    [
        (u >> 24 & 0xff) as u8,
        (u >> 16 & 0xff) as u8,
        (u >> 8 & 0xff) as u8,
        (u & 0xff) as u8,
    ]
}
