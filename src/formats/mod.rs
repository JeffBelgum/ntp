use std::fmt;

pub mod timestamp;


#[repr(u8)]
#[deriving(Show,FromPrimitive,PartialEq)]
pub enum LeapIndicator {
    NoWarning = 0,
    AddOne = 1,
    SubOne = 2,
    Unknown = 3
}

#[repr(u8)]
#[deriving(Show,FromPrimitive,PartialEq,Eq,PartialOrd,Ord)]
pub enum Version {
    Ver1 = 1,
    Ver2 = 2,
    Ver3 = 3,
    Ver4 = 4,
}

#[repr(u8)]
#[deriving(Show,FromPrimitive,PartialEq)]
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

#[deriving(Show,PartialEq,Eq,PartialOrd,Ord)]
pub struct Stratum {
    value: u8,
}

impl Stratum {
    #[inline]
    pub fn new(val: u8) -> Stratum { Stratum { value: val } }
    #[inline]
    pub fn unspecified(&self) -> bool { self.value == 0 }
    #[inline]
    pub fn invalid(&self) -> bool { self.value == 0 }
    #[inline]
    pub fn primary(&self) -> bool { self.value == 1 }
    #[inline]
    pub fn secondary(&self) -> bool { 2 <= self.value && self.value <= 15 }
    #[inline]
    pub fn unsynchronized(&self) -> bool { self.value == 16 }
    #[inline]
    pub fn reserved(&self) -> bool { self.value >= 17 }
    #[inline]
    pub fn get_value(&self) -> u8 { self.value }
}


/// ascii chars packed into a u32 for matching a raw buffer
#[repr(u32)]
#[deriving(FromPrimitive,PartialEq)]
pub enum ReferenceIdentifier {
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

impl fmt::Show for ReferenceIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match *self {
            GOES => "GOES: Geosynchronous Orbit Environment Satellite",
            GPS  => "GPS: Global Position System",
            CDMA => "CDMA: Code Division Multiple Access",
            GAL  => "GAL: Galileo Positioning System",
            PPS  => "PPS: Generic pulse-per-second",
            IRIG => ": Inter-Range Instrumentation Group",
            WWVB => ": LF Radio WWVB Ft. Collins, CO 60 kHz",
            DCF  => ": LF Radio DCF77 Mainflingen, DE 77.5 kHz",
            HBG  => ": LF Radio HBG Prangins, HB 75 kHz",
            MSF  => ": LF Radio MSF Anthorn, UK 60 kHz",
            JJY  => ": LF Radio JJY Fukushima, JP 40 kHz, Saga, JP 60 kHz",
            LORC => ": MF Radio LORAN C station, 100 kHz",
            TDF  => ": MF Radio Allouis, FR 162 kHz",
            CHU  => ": HF Radio CHU Ottawa, Ontario",
            WWV  => ": HF Radio WWV Ft. Collins, CO",
            WWVH => ": HF Radio WWVH Kauai, HI",
            NIST => ": NIST telephone modem",
            ACTS => ": NIST telephone modem",
            USNO => ": USNO telephone modem",
            PTB  => ": European telephone modem",
            LOCL => ": Not Yet Described",
            CESM => ": Not Yet Described",
            RBDM => ": Not Yet Described",
            OMEG => ": Not Yet Described",
            DCN  => ": Not Yet Described",
            TSP  => ": Not Yet Described",
            DTS  => ": Not Yet Described",
            ATOM => ": Not Yet Described",
            VLF  => ": Not Yet Described",
            OPPS => ": Not Yet Described",
            FREE => ": Not Yet Described",
            INIT => ": Not Yet Described",
            NULL => ": Not Yet Described",
        };
        write!(f, "{}", desc)
    }
}

