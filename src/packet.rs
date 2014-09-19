extern crate time;

use std::io::{MemReader, MemWriter};

use super::formats::{LeapIndicator, Version, Mode, Stratum, ReferenceIdentifier};
use super::formats::timestamp::{ShortFormat, TimestampFormat};

macro_rules! unwrap_or_return(
        ($v:expr, $e:expr) => (match $v { Some(v) => v, None => return $e })
        )


#[deriving(Show, PartialEq)]
pub struct Packet {
    pub li:             LeapIndicator,
    pub vn:             Version,
    pub mode:           Mode,
    pub stratum:        Stratum,
    pub poll:           i8,
    pub precision:      i8,
    pub delay:          ShortFormat,
    pub dispersion:     ShortFormat,
    pub ref_id:         ReferenceIdentifier,
    pub ref_time:       TimestampFormat,
    pub orig_time:      TimestampFormat,
    pub recv_time:      TimestampFormat,
    pub transmit_time:  TimestampFormat,
}

impl Packet {

    /// Data on the wire is big endian
    pub fn from_bytes(buf: &[u8, ..48]) -> Result<Packet, &'static str> {
        let mut reader = MemReader::new(buf.to_vec());
        let packed_li_vn_mode = reader.read_u8().unwrap();
        
        Ok(Packet { 
            li:   unwrap_or_return!(FromPrimitive::from_u8(packed_li_vn_mode >> 6),
                                    Err("Invalid Leap Indicator")),
            vn:   unwrap_or_return!(FromPrimitive::from_u8(packed_li_vn_mode >> 3 & 0b111),
                                    Err("Invalid Version")),
            mode: unwrap_or_return!(FromPrimitive::from_u8(packed_li_vn_mode & 0b111),
                                    Err("Invalid Mode")),
            stratum: Stratum::new(reader.read_u8().unwrap()),
            poll: reader.read_i8().unwrap(),
            precision: reader.read_i8().unwrap(),
            delay: ShortFormat { 
                sec: reader.read_be_u16().unwrap(),
                frac: reader.read_be_u16().unwrap(),
            },
            dispersion: ShortFormat { 
                sec: reader.read_be_u16().unwrap(),
                frac: reader.read_be_u16().unwrap(),
            },
            ref_id: unwrap_or_return!(FromPrimitive::from_u32(reader.read_be_u32().unwrap()),
                                      Err("Invalid RefId")),
            ref_time: TimestampFormat { 
                sec: reader.read_be_u32().unwrap(),
                frac: reader.read_be_u32().unwrap(),
            },
            orig_time: TimestampFormat { 
                sec: reader.read_be_u32().unwrap(),
                frac: reader.read_be_u32().unwrap(),
            },
            recv_time: TimestampFormat { 
                sec: reader.read_be_u32().unwrap(),
                frac: reader.read_be_u32().unwrap(),
            },
            transmit_time: TimestampFormat { 
                sec: reader.read_be_u32().unwrap(),
                frac: reader.read_be_u32().unwrap(),
            },

        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = MemWriter::with_capacity(48);
        let packed_byte = (self.mode as u8) | (self.vn as u8) << 3 | (self.li as u8) << 6;
        writer.write_u8(packed_byte).unwrap();
        writer.write_u8(self.stratum.get_value()).unwrap();
        writer.write_i8(self.poll).unwrap();
        writer.write_i8(self.precision).unwrap();
        writer.write_be_u16(self.delay.sec).unwrap();
        writer.write_be_u16(self.delay.frac).unwrap();
        writer.write_be_u16(self.dispersion.sec).unwrap();
        writer.write_be_u16(self.dispersion.frac).unwrap();
        writer.write_be_u32(self.ref_id as u32).unwrap();
        writer.write_be_u32(self.ref_time.sec).unwrap();
        writer.write_be_u32(self.ref_time.frac).unwrap();
        writer.write_be_u32(self.orig_time.sec).unwrap();
        writer.write_be_u32(self.orig_time.frac).unwrap();
        writer.write_be_u32(self.recv_time.sec).unwrap();
        writer.write_be_u32(self.recv_time.frac).unwrap();
        writer.write_be_u32(self.transmit_time.sec).unwrap();
        writer.write_be_u32(self.transmit_time.frac).unwrap();

        writer.unwrap()
    }
}





#[test]
fn packet_from_bytes() {
    use super::formats::{NoWarning,Ver2,Server,CDMA};

    let input = [20u8, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169, 46, 99,
    215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215, 188, 128, 113,
    46, 35, 158, 108];
    let expected_output = Packet { li: NoWarning, vn: Ver2, mode: Server, stratum: Stratum::new(1), 
                          poll: 3, precision: -16, delay: ShortFormat { sec: 0, frac: 0 }, 
                          dispersion:    ShortFormat     { sec: 0, frac: 24 }, ref_id: CDMA, 
                          ref_time:      TimestampFormat { sec: 3619455081, frac: 3332976227 }, 
                          orig_time:     TimestampFormat { sec: 3619402178, frac: 2670688256 }, 
                          recv_time:     TimestampFormat { sec: 3619455089, frac: 770500141 }, 
                          transmit_time: TimestampFormat { sec: 3619455089, frac: 774086252 } 
                        };

    assert_eq!(expected_output, Packet::from_bytes(&input).unwrap());

}

#[test]
fn packet_to_bytes() {
    use super::formats::{NoWarning,Ver2,Server,CDMA};

    let expected_output = vec![20u8, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169, 46, 99,
    215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215, 188, 128, 113,
    46, 35, 158, 108];
    let input = Packet { li: NoWarning, vn: Ver2, mode: Server, stratum: Stratum::new(1), 
                          poll: 3, precision: -16, delay: ShortFormat { sec: 0, frac: 0 }, 
                          dispersion:    ShortFormat     { sec: 0, frac: 24 }, ref_id: CDMA, 
                          ref_time:      TimestampFormat { sec: 3619455081, frac: 3332976227 }, 
                          orig_time:     TimestampFormat { sec: 3619402178, frac: 2670688256 }, 
                          recv_time:     TimestampFormat { sec: 3619455089, frac: 770500141 }, 
                          transmit_time: TimestampFormat { sec: 3619455089, frac: 774086252 } 
                        };
    assert_eq!(input.to_bytes(), expected_output);

}

#[test]
fn packet_conversion_roundtrip() {
    let input = [20u8, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169, 46, 99,
    215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215, 188, 128, 113,
    46, 35, 158, 108];
    let output = Packet::from_bytes(&input).unwrap().to_bytes();
    assert_eq!(input.as_slice(), output.as_slice());
}
