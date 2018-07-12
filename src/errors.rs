//! Error type for the ntp crate.
use conv::Unrepresentable;
use std::io;

error_chain! {
    foreign_links {
        IOError(io::Error);
        UnrepresentableU8(Unrepresentable<u8>);
        UnrepresentableU32(Unrepresentable<u32>);
    }
}
