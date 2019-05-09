extern crate ntp;

use ntp::protocol::{
    ConstPackedSizeBytes, LeapIndicator, Mode, Packet, PrimarySource, ReadBytes,
    ReferenceIdentifier, ShortFormat, Stratum, TimestampFormat, Version, WriteBytes,
};

#[test]
fn packet_from_bytes() {
    let input = [
        20u8, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169, 46,
        99, 215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215, 188,
        128, 113, 46, 35, 158, 108,
    ];
    let expected_output = Packet {
        leap_indicator: LeapIndicator::NoWarning,
        version: Version::V2,
        mode: Mode::Server,
        stratum: Stratum::PRIMARY,
        poll: 3,
        precision: -16,
        root_delay: ShortFormat {
            seconds: 0,
            fraction: 0,
        },
        root_dispersion: ShortFormat {
            seconds: 0,
            fraction: 24,
        },
        reference_id: ReferenceIdentifier::PrimarySource(PrimarySource::CDMA),
        reference_timestamp: TimestampFormat {
            seconds: 3619455081,
            fraction: 3332976227,
        },
        origin_timestamp: TimestampFormat {
            seconds: 3619402178,
            fraction: 2670688256,
        },
        receive_timestamp: TimestampFormat {
            seconds: 3619455089,
            fraction: 770500141,
        },
        transmit_timestamp: TimestampFormat {
            seconds: 3619455089,
            fraction: 774086252,
        },
    };

    let packet = (&input[..]).read_bytes::<Packet>().unwrap();
    assert_eq!(expected_output, packet);
}

#[test]
fn packet_to_bytes() {
    let expected_output = [
        20, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169, 46,
        99, 215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215, 188,
        128, 113, 46, 35, 158, 108,
    ];
    let input = Packet {
        leap_indicator: LeapIndicator::NoWarning,
        version: Version::V2,
        mode: Mode::Server,
        stratum: Stratum::PRIMARY,
        poll: 3,
        precision: -16,
        root_delay: ShortFormat {
            seconds: 0,
            fraction: 0,
        },
        root_dispersion: ShortFormat {
            seconds: 0,
            fraction: 24,
        },
        reference_id: ReferenceIdentifier::PrimarySource(PrimarySource::CDMA),
        reference_timestamp: TimestampFormat {
            seconds: 3619455081,
            fraction: 3332976227,
        },
        origin_timestamp: TimestampFormat {
            seconds: 3619402178,
            fraction: 2670688256,
        },
        receive_timestamp: TimestampFormat {
            seconds: 3619455089,
            fraction: 770500141,
        },
        transmit_timestamp: TimestampFormat {
            seconds: 3619455089,
            fraction: 774086252,
        },
    };
    let mut bytes = [0u8; Packet::PACKED_SIZE_BYTES];
    (&mut bytes[..]).write_bytes(&input).unwrap();
    assert_eq!(&bytes[..], &expected_output[..]);
}

#[test]
fn packet_conversion_roundtrip() {
    let input = [
        20, 1, 3, 240, 0, 0, 0, 0, 0, 0, 0, 24, 67, 68, 77, 65, 215, 188, 128, 105, 198, 169, 46,
        99, 215, 187, 177, 194, 159, 47, 120, 0, 215, 188, 128, 113, 45, 236, 230, 45, 215, 188,
        128, 113, 46, 35, 158, 108,
    ];
    let packet = (&input[..]).read_bytes::<Packet>().unwrap();
    let mut output = [0u8; Packet::PACKED_SIZE_BYTES];
    (&mut output[..]).write_bytes(&packet).unwrap();
    assert_eq!(&input[..], &output[..]);
}
