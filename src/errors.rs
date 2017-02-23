use std::io;
use conv::Unrepresentable;

// TODO use newer syntax for foreign_links section
// generate error types for this crate using https://github.com/brson/error-chain
error_chain! { 
    foreign_links {
        io::Error, IOError;
        Unrepresentable<u8>, UnrepresentableU8;
        Unrepresentable<u32>, UnrepresentableU32;
    }
}
