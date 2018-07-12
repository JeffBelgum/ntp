use std::fmt;

pub mod timestamp;

custom_derive! {
    #[repr(u8)]
    #[derive(Debug,PartialEq,TryFrom(u8))]
    pub enum LeapIndicator {
        NoWarning = 0,
        AddOne = 1,
        SubOne = 2,
        Unknown = 3
    }
}

impl Default for LeapIndicator {
    fn default() -> LeapIndicator {
        LeapIndicator::NoWarning
    }
}

custom_derive! {
    #[repr(u8)]
    #[derive(Debug,PartialEq,Eq,PartialOrd,Ord,TryFrom(u8))]
    pub enum Version {
        Ver1 = 1,
        Ver2 = 2,
        Ver3 = 3,
        Ver4 = 4,
    }
}

impl Default for Version {
    fn default() -> Version {
        Version::Ver2
    }
}

custom_derive! {
    #[repr(u8)]
    #[derive(Debug,PartialEq,TryFrom(u8))]
    pub enum Mode {
        Reserved = 0,
        SymmetricActive = 1,
        SymmetricPassive = 2,
        Client = 3,
        Server = 4,
        Broadcase = 5,
        NTPControlMessage = 6,
        ReservedForPrivateUse = 7,
    }
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Reserved
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Default, Ord)]
pub struct Stratum {
    value: u8,
}

impl Stratum {
    #[inline]
    pub fn new(val: u8) -> Stratum {
        Stratum { value: val }
    }
    #[inline]
    pub fn unspecified(&self) -> bool {
        self.value == 0
    }
    #[inline]
    pub fn invalid(&self) -> bool {
        self.value == 0
    }
    #[inline]
    pub fn primary(&self) -> bool {
        self.value == 1
    }
    #[inline]
    pub fn secondary(&self) -> bool {
        2 <= self.value && self.value <= 15
    }
    #[inline]
    pub fn unsynchronized(&self) -> bool {
        self.value == 16
    }
    #[inline]
    pub fn reserved(&self) -> bool {
        self.value >= 17
    }
    #[inline]
    pub fn get_value(&self) -> u8 {
        self.value
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ReferenceIdentifier {
    Primary(PrimarySource),
    Secondary(u32),
}

impl Default for ReferenceIdentifier {
    fn default() -> ReferenceIdentifier {
        ReferenceIdentifier::Primary(Default::default())
    }
}

impl From<ReferenceIdentifier> for u32 {
    fn from(ri: ReferenceIdentifier) -> u32 {
        match ri {
            ReferenceIdentifier::Primary(s) => s as u32,
            ReferenceIdentifier::Secondary(s) => s as u32,
        }
    }
}

impl fmt::Display for ReferenceIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReferenceIdentifier::Primary(id) => write!(f, "{}", id),
            ReferenceIdentifier::Secondary(id) => {
                let fields: Vec<u8> = vec![
                    (id >> 24 & 0xff) as u8,
                    (id >> 16 & 0xff) as u8,
                    (id >> 8 & 0xff) as u8,
                    (id & 0xff) as u8,
                ];
                write!(f, "{}.{}.{}.{}", fields[0], fields[1], fields[2], fields[3])
            }
        }
    }
}

/// ascii chars packed into a u32 for matching a raw buffer
custom_derive! {
    #[repr(u32)]
    #[derive(Debug,PartialEq,Copy,Clone,TryFrom(u32))]
    pub enum PrimarySource {
        GOES = 1196377427,
        GPS  = 1196446464,
        CDMA = 1128549697,
        GAL  = 1195461632,
        PPS  = 1347441408,
        IRIG = 1230129479,
        WWVB = 1465341506,
        DCF  = 1145259520,
        HBG  = 1212303104,
        MSF  = 1297303040,
        JJY  = 1246386432,
        LORC = 1280266819,
        TDF  = 1413760512,
        CHU  = 1128813824,
        WWV  = 1465341440,
        WWVH = 1465341512,
        NIST = 1313428308,
        ACTS = 1094931539,
        USNO = 1431522895,
        PTB  = 1347699200,
        GOOG = 1196379975,
        LOCL = 1280262988,
        CESM = 1128616781,
        RBDM = 1380074573,
        OMEG = 1330464071,
        DCN  = 1145261568,
        TSP  = 1414746112,
        DTS  = 1146376960,
        ATOM = 1096044365,
        VLF  = 1447839232,
        OPPS = 1330663507,
        FREE = 1179796805,
        INIT = 1229867348,
        NULL = 0000000000,
    }
}

impl Default for PrimarySource {
    fn default() -> PrimarySource {
        PrimarySource::NULL
    }
}

impl fmt::Display for PrimarySource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PrimarySource::*;
        let desc = match *self {
            GOES => "GOES: Geosynchronous Orbit Environment Satellite",
            GPS => "GPS: Global Position System",
            CDMA => "CDMA: Code Division Multiple Access",
            GAL => "GAL: Galileo Positioning System",
            PPS => "PPS: Generic pulse-per-second",
            IRIG => "IRIG: Inter-Range Instrumentation Group",
            WWVB => "WWVB: LF Radio WWVB Ft. Collins, CO 60 kHz",
            DCF => "DCF: LF Radio DCF77 Mainflingen, DE 77.5 kHz",
            HBG => "HBG: LF Radio HBG Prangins, HB 75 kHz",
            MSF => "MSF: LF Radio MSF Anthorn, UK 60 kHz",
            JJY => "JJY: LF Radio JJY Fukushima, JP 40 kHz, Saga, JP 60 kHz",
            LORC => "LORC: MF Radio LORAN C station, 100 kHz",
            TDF => "TDF: MF Radio Allouis, FR 162 kHz",
            CHU => "CHU: HF Radio CHU Ottawa, Ontario",
            WWV => "WWV: HF Radio WWV Ft. Collins, CO",
            WWVH => "WWVH: HF Radio WWVH Kauai, HI",
            NIST => "NIST: NIST telephone modem",
            ACTS => "ACTS: ACTS telephone modem",
            USNO => "USNO: USNO telephone modem",
            PTB => "PTB: European telephone modem",
            GOOG => "GOOG: Google Public NTP",
            LOCL => "LOCL: Not Yet Described",
            CESM => "CESM: Not Yet Described",
            RBDM => "RBDM: Not Yet Described",
            OMEG => "OMEG: Not Yet Described",
            DCN => "DCN: Not Yet Described",
            TSP => "TSP: Not Yet Described",
            DTS => "DTS: Not Yet Described",
            ATOM => "ATOM: Not Yet Described",
            VLF => "VLF: Not Yet Described",
            OPPS => "OPPS: Not Yet Described",
            FREE => "FREE: Not Yet Described",
            INIT => "INIT: Not Yet Described",
            NULL => "NULL: Null Value",
        };
        write!(f, "{}", desc)
    }
}
