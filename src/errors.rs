use std::io;

error_chain! { 
    foreign_links {
        IOError(io::Error);
    }
}
